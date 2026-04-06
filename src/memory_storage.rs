use crate::*;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

pub struct MemoryStorage<B: Backend> {
    index_manager: IndexManager,
    entries: Vec<B::StoredValue>,
    sync_memory: SyncMemoryStorage<B>,
}
impl<B: Backend> MemoryStorage<B> {
    pub fn new(sync_memory: SyncMemoryStorage<B>) -> Self {
        MemoryStorage {
            index_manager: IndexManager::new(),
            entries: Vec::new(),
            sync_memory,
        }
    }

    pub(crate) fn get_index_manager(&self) -> IndexManager {
        self.index_manager.clone()
    }

    pub fn extract_deletes(&mut self) -> (Vec<B::StoredValue>, Vec<B::SendStoredValue>) {
        let mut unreferenced = Vec::new();
        self.index_manager
            .extract_unreferenced_to(&mut unreferenced);

        // Swap out the entries that are no longer referenced
        let deleted = unreferenced
            .into_iter()
            .map(|i| std::mem::replace(&mut self.entries[i], B::deleted_value()))
            .collect::<Vec<_>>();

        (deleted, self.sync_memory.extract_drop())
    }

    pub fn flush_deletes(&mut self) {
        let mut unreferenced = Vec::new();
        self.index_manager
            .extract_unreferenced_to(&mut unreferenced);

        // Discard the entries that are no longer referenced
        for index in unreferenced.iter().cloned() {
            if let Some(entry) = self.entries.get_mut(index) {
                *entry = B::deleted_value();
            }
        }

        // Clear the sync memory drop queue
        self.sync_memory.extract_drop();
    }

    pub fn sync_memory(&self) -> &SyncMemoryStorage<B> {
        &self.sync_memory
    }

    pub fn store_send(&mut self, value: B::SendStoredValue) -> StoredSendData<B> {
        StoredSendData::new(value, &self.sync_memory)
    }

    pub fn store(&mut self, value: B::StoredValue) -> StoredData<B> {
        let v = self.index_manager.next_index::<B>();
        match self.entries.get_mut(v.storage_index()) {
            Some(existing) => {
                *existing = value;
            }
            None => {
                self.entries.push(value);
            }
        }
        v
    }

    pub fn try_take(&mut self, data: StoredData<B>) -> Result<B::StoredValue, StoredData<B>> {
        assert!(
            data.device_id() == self.index_manager.device_id(),
            "StoredData does not belong to this MemoryStorage"
        );
        if self.index_manager.try_take(data.storage_index()) {
            let result =
                std::mem::replace(&mut self.entries[data.storage_index()], B::deleted_value());
            data.forget(); // Forget the data, so it won't be decremented again
            Ok(result)
        } else {
            Err(data)
        }
    }

    pub fn get<'a>(&'a self, data: &StoredData<B>) -> &'a B::StoredValue {
        assert!(
            data.device_id() == self.index_manager.device_id(),
            "StoredData does not belong to this MemoryStorage"
        );
        self.entries
            .get(data.storage_index())
            .expect("Invalid storage index")
    }

    pub fn get_mut<'a>(&'a mut self, data: &StoredData<B>) -> &'a mut B::StoredValue {
        assert!(
            data.device_id() == self.index_manager.device_id(),
            "StoredData does not belong to this MemoryStorage"
        );
        self.entries
            .get_mut(data.storage_index())
            .expect("Invalid storage index")
    }
}

#[derive(Clone)]
pub(crate) struct IndexManager(Rc<RefCell<IndexManagerInner>>);
impl IndexManager {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(IndexManagerInner {
            device_id: DeviceId::next(),
            recent_unreferenced: Vec::new(),
            available_indexes: Vec::new(),
            ref_counts: Vec::new(),
            device: None,
        })))
    }

    pub(crate) fn device_id(&self) -> DeviceId {
        self.0.borrow().device_id
    }

    pub(crate) fn set_device<B: Backend>(&self, device: Device<B>) {
        let mut inner = self.0.borrow_mut();
        inner.device = Some(Box::new(device));
    }

    /// Returns the next available index, or a new index if none are available.
    ///
    /// The index will start with a reference count of 1.
    pub(crate) fn next_index<B: Backend>(&self) -> StoredData<B> {
        let mut inner = self.0.borrow_mut();
        let device = inner
            .device
            .as_ref()
            .and_then(|d| d.downcast_ref::<Device<B>>().cloned())
            .expect("Device must be fully initialized before storing data (place initialization logic in `Backend::initialize_device_and_surface`)");

        if let Some(index) = inner.available_indexes.pop() {
            inner.ref_counts[index] = 1;
            StoredData::new(device, index)
        } else {
            let index = inner.ref_counts.len();
            inner.ref_counts.push(1);
            StoredData::new(device, index)
        }
    }

    pub(crate) fn extract_unreferenced_to(&self, unreferenced: &mut Vec<usize>) {
        let mut inner = self.0.borrow_mut();
        if !inner.recent_unreferenced.is_empty() {
            unreferenced.extend_from_slice(&inner.recent_unreferenced);
            inner.recent_unreferenced.clear();

            inner.available_indexes.extend_from_slice(&unreferenced);
        }
    }

    pub(crate) fn increment(&self, index: usize) {
        let mut inner = self.0.borrow_mut();

        let existing = inner.ref_counts.get(index).cloned().unwrap_or(0);
        assert!(
            existing > 0,
            "Cannot increment a reference count that is already zero"
        );
        inner.ref_counts[index] = existing + 1;
    }

    pub(crate) fn decrement(&self, index: usize) {
        let mut inner = self.0.borrow_mut();

        let existing = inner.ref_counts.get(index).cloned().unwrap_or(0);
        if existing == 0 {
            // Special case: If the index is usize::MAX, it is treated as a special forgotten index
            if index == usize::MAX {
                return;
            }
            panic!("Cannot decrement a reference count that is already zero");
        }
        inner.ref_counts[index] = existing - 1;
        if existing == 1 {
            inner.recent_unreferenced.push(index);
        }
    }

    pub(crate) fn try_take(&self, index: usize) -> bool {
        let mut inner = self.0.borrow_mut();

        if inner.ref_counts.get(index).cloned().unwrap_or(0) == 1 {
            // If the reference count is 1, we can take it
            // When taking data, skip the `recent_unreferenced` queue and go directly to available indexes
            inner.ref_counts[index] = 0;
            inner.available_indexes.push(index);
            true
        } else {
            false
        }
    }
}

struct IndexManagerInner {
    device_id: DeviceId,
    recent_unreferenced: Vec<usize>,
    available_indexes: Vec<usize>,
    ref_counts: Vec<usize>,
    device: Option<Box<dyn std::any::Any>>,
}

pub struct SyncMemoryStorage<B: Backend>(
    std::sync::Arc<std::sync::Mutex<SyncMemoryStorageInner<B>>>,
);
impl<B: Backend> Clone for SyncMemoryStorage<B> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<B: Backend> SyncMemoryStorage<B> {
    pub(crate) fn new() -> Self {
        Self(std::sync::Arc::new(std::sync::Mutex::new(
            SyncMemoryStorageInner {
                drop_queue: Vec::new(),
            },
        )))
    }

    pub fn store(&self, value: B::SendStoredValue) -> StoredSendData<B> {
        StoredSendData::new(value, self)
    }

    pub(crate) fn push(&self, value: B::SendStoredValue) {
        let mut inner = self.0.lock().unwrap();
        inner.drop_queue.push(value);
    }

    pub(crate) fn extract_drop(&self) -> Vec<B::SendStoredValue> {
        let mut inner = self.0.lock().unwrap();
        std::mem::take(&mut inner.drop_queue)
    }
}

struct SyncMemoryStorageInner<B: Backend> {
    drop_queue: Vec<B::SendStoredValue>,
}

static GLOBAL_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DeviceId(u64);
impl DeviceId {
    fn next() -> Self {
        let id = GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        DeviceId(id)
    }
}
