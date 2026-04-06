use crate::*;

pub struct StoredData<B: Backend> {
    device: Device<B>,
    index: usize,
}
impl<B: Backend> StoredData<B> {
    pub(crate) fn new(device: Device<B>, index: usize) -> Self {
        StoredData { device, index }
    }
    pub(crate) fn device_id(&self) -> DeviceId {
        self.device.device_id()
    }

    pub fn device(&self) -> &Device<B> {
        &self.device
    }

    /// If this is the only reference to this data, it will be converted to a `StoredSendData`.
    pub fn into_send(self) -> Result<StoredSendData<B>, B::Error> {
        self.device()
            .clone()
            .internal(|d| d.foreground_to_background(self))
    }

    pub(crate) fn storage_index(&self) -> usize {
        self.index
    }

    pub(crate) fn forget(mut self) {
        self.index = usize::MAX; // Mark as forgotten
    }

    pub unsafe fn unchecked_cast<T>(self) -> Stored<B, T> {
        Stored {
            storage: self,
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates an index into the underlying data. Will return an error if indexing is not supported by the underlying data type
    /// or if the index is out of bounds.
    ///
    /// This will hold a reference to the underlying data, so it will not be dropped until this index is also dropped.
    pub fn try_index(&self, index: usize) -> Result<StoredData<B>, B::Error> {
        self.device.internal(|d| d.try_index(self, index))
    }
    /// Creates a slice of the underlying data. Will return an error if slicing is not supported by the underlying data type or
    /// if the slice is out of bounds.
    ///
    /// This will hold a reference to the underlying data, so it will not be dropped until this slice is also dropped.
    pub fn try_slice<Bounds: std::ops::RangeBounds<usize>>(
        &self,
        range: Bounds,
    ) -> Result<StoredData<B>, B::Error> {
        self.device.internal(|d| d.try_slice(self, range))
    }
    /// Returns the length of the data. If this is sliced, then the slice length will be returned.
    pub fn len(&self) -> usize {
        self.device.internal(|d| d.len(self))
    }
}
impl<B: Backend> Clone for StoredData<B> {
    fn clone(&self) -> Self {
        let device = self.device.clone();
        let index = self.index;
        device.index_manager().increment(index);
        StoredData { device, index }
    }
}
impl<B: Backend> Drop for StoredData<B> {
    fn drop(&mut self) {
        // Note: The decrement method will handle the case where the forgotten index is usize::MAX
        self.device.index_manager().decrement(self.index);
    }
}
impl<B: Backend> PartialEq for StoredData<B> {
    fn eq(&self, other: &Self) -> bool {
        self.storage_index() == other.storage_index() && self.device_id() == other.device_id()
    }
}
impl<B: Backend> Eq for StoredData<B> {}
impl<B: Backend> std::hash::Hash for StoredData<B> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.storage_index().hash(state);
        self.device_id().hash(state);
    }
}
impl<B: Backend> std::fmt::Debug for StoredData<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.device.try_internal(|d| match d {
            Some(d) => f.debug_tuple("Stored").field(d.debug_value(self)).finish(),
            None => f
                .debug_tuple("StoredAt")
                .field(&self.storage_index())
                .finish(),
        })
    }
}
impl<B: Backend, T> From<Stored<B, T>> for StoredData<B> {
    fn from(value: Stored<B, T>) -> Self {
        value.into_stored_data()
    }
}
impl<'a, B: Backend, T> From<&'a Stored<B, T>> for StoredData<B> {
    fn from(value: &'a Stored<B, T>) -> Self {
        value.stored_data().clone()
    }
}
impl<'a, B: Backend> From<&'a StoredData<B>> for StoredData<B> {
    fn from(value: &'a StoredData<B>) -> Self {
        value.clone()
    }
}

pub struct Stored<B: Backend, T> {
    storage: StoredData<B>,
    _marker: std::marker::PhantomData<T>,
}
impl<B: Backend, T> Stored<B, T> {
    pub fn device(&self) -> &Device<B> {
        self.storage.device()
    }

    /// Reloads the data with new values.
    ///
    /// Will return an error if reloading is not supported by the underlying data type.
    pub fn reload<V>(&self, value: &V) -> Result<(), B::Error>
    where
        V: DeviceLoad<B, LoadAs = T>,
    {
        let value = std::slice::from_ref(value);
        self.device().internal(|d| V::reload(d, value, self))
    }

    /// Reloads an array of data. If the underlying storage is too small, it will be re-allocated to fit the new data.
    ///
    /// Will return an error if reloading is not supported by the underlying data type.
    pub fn reload_array<V>(&self, value: &[V]) -> Result<(), B::Error>
    where
        V: DeviceLoad<B, LoadAs = T>,
    {
        self.device().internal(|d| V::reload(d, value, self))
    }

    pub fn stored_data(&self) -> &StoredData<B> {
        &self.storage
    }
    pub fn into_stored_data(self) -> StoredData<B> {
        self.storage
    }

    /// Creates an index into the underlying array. Will panic if indexing is not supported by the underlying data type or if the
    /// index is out of bounds.
    ///
    /// This will return a `Stored<IndexIn<T>>`, use `index()` if you want to get the value type instead of the index type.
    ///
    /// This will hold a reference to the underlying array, so it will not be dropped until this index is also dropped.
    #[track_caller]
    pub fn index_any(&self, index: usize) -> Stored<B, IndexIn<T>> {
        self.try_index_any(index).expect("valid indexing")
    }
    /// Creates an index into the underlying array. Will return an error if indexing is not supported by the underlying data type
    /// or if the index is out of bounds.
    ///
    /// This will return a `Stored<IndexIn<T>>`, use `index()` if you want to get the value type instead of the index type.
    ///
    /// This will hold a reference to the underlying array, so it will not be dropped until this index is also dropped.
    pub fn try_index_any(&self, index: usize) -> Result<Stored<B, IndexIn<T>>, B::Error> {
        self.storage
            .try_index(index)
            .map(|v| unsafe { v.unchecked_cast() })
    }

    /// Creates an index into the underlying data and re-casts it as the underlying value type. Will panic if indexing is not
    /// supported by the underlying data type or if the index is out of bounds.
    ///
    /// This will hold a reference to the underlying data, so it will not be dropped until this index is also dropped.
    #[track_caller]
    pub fn index(&self, index: usize) -> Stored<B, T::Value>
    where
        T: IndexAsType,
    {
        self.try_index(index).expect("valid indexing")
    }
    /// Creates an index into the underlying data and re-casts it as the underlying value type. Will return an error if indexing
    /// is not supported by the underlying data type or if the index is out of bounds.
    ///
    /// This will hold a reference to the underlying data, so it will not be dropped until this index is also dropped.
    pub fn try_index(&self, index: usize) -> Result<Stored<B, T::Value>, B::Error>
    where
        T: IndexAsType,
    {
        self.storage
            .try_index(index)
            .map(|v| unsafe { v.unchecked_cast() })
    }

    /// Creates a slice of the underlying data. Will panic if slicing is not supported by the underlying data type or if the
    /// slice is out of bounds.
    ///
    /// This will hold a reference to the underlying data, so it will not be dropped until this slice is also dropped.
    #[track_caller]
    pub fn slice<Bounds: std::ops::RangeBounds<usize>>(&self, range: Bounds) -> Stored<B, T> {
        self.try_slice(range).expect("valid slicing")
    }
    /// Creates a slice of the underlying data. Will return an error if slicing is not supported by the underlying data type or
    /// if the slice is out of bounds.
    ///
    /// This will hold a reference to the underlying data, so it will not be dropped until this slice is also dropped.
    pub fn try_slice<Bounds: std::ops::RangeBounds<usize>>(
        &self,
        range: Bounds,
    ) -> Result<Stored<B, T>, B::Error> {
        self.storage
            .try_slice(range)
            .map(|v| unsafe { v.unchecked_cast() })
    }
    /// Returns the length of the data. If this is indexed/sliced, then the index/slice length will be returned.
    pub fn len(&self) -> usize {
        self.storage.len()
    }
}
impl<B: Backend, T> Clone for Stored<B, T> {
    fn clone(&self) -> Self {
        let storage = self.storage.clone();
        Stored {
            storage,
            _marker: std::marker::PhantomData,
        }
    }
}
impl<B: Backend, T> PartialEq for Stored<B, T> {
    fn eq(&self, other: &Self) -> bool {
        self.storage == other.storage
    }
}
impl<B: Backend, T> Eq for Stored<B, T> {}
impl<B: Backend, T> std::hash::Hash for Stored<B, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.storage.hash(state);
    }
}
impl<B: Backend, T> std::fmt::Debug for Stored<B, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.storage.fmt(f)
    }
}

pub struct IndexIn<T>(std::marker::PhantomData<T>);

pub trait IndexAsType {
    type Value;
}

pub struct StoredSend<B: Backend, T> {
    storage: StoredSendData<B>,
    _marker: std::marker::PhantomData<T>,
}
impl<B: Backend, T> StoredSend<B, T> {
    pub fn finish(self, device: &Device<B>) -> Result<Stored<B, T>, B::Error> {
        let result = self.storage.finish(device)?;
        unsafe { Ok(result.unchecked_cast::<T>()) }
    }

    pub fn stored_data(&self) -> &StoredSendData<B> {
        &self.storage
    }
    pub fn into_stored_data(self) -> StoredSendData<B> {
        self.storage
    }
}
impl<B: Backend, T> std::fmt::Debug for StoredSend<B, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.storage.fmt(f)
    }
}

pub struct StoredSendData<B: Backend> {
    value: Option<B::SendStoredValue>,
    sync_memory: SyncMemoryStorage<B>,
}
impl<B: Backend> StoredSendData<B> {
    pub(crate) fn new(value: B::SendStoredValue, sync_memory: &SyncMemoryStorage<B>) -> Self {
        StoredSendData {
            value: Some(value),
            sync_memory: sync_memory.clone(),
        }
    }

    pub fn finish(mut self, device: &Device<B>) -> Result<StoredData<B>, B::Error> {
        let value = self.value.take().unwrap();
        device.internal(|d| d.background_to_foreground(value))
    }

    pub unsafe fn unchecked_cast<T>(self) -> StoredSend<B, T> {
        StoredSend {
            storage: self,
            _marker: std::marker::PhantomData,
        }
    }
}
impl<B: Backend> Drop for StoredSendData<B> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.sync_memory.push(value);
        }
    }
}
impl<B: Backend> std::fmt::Debug for StoredSendData<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => f.debug_tuple("StoredSend").field(value).finish(),
            None => f.debug_tuple("StoredSend").field(&"*Dropped*").finish(),
        }
    }
}
