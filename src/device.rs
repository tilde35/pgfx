use crate::*;
use std::{cell::RefCell, rc::Rc};

pub struct Device<B: Backend> {
    data: Rc<(RefCell<B::DeviceInstance>, IndexManager)>,
}
impl<B: Backend> Device<B> {
    pub(crate) fn new(data: B::DeviceInstance, index_manager: IndexManager) -> Self {
        let device = Device {
            data: Rc::new((RefCell::new(data), index_manager)),
        };

        // Finish index manager setup (it needs a device reference to store data)
        device.index_manager().set_device(device.clone());

        device
    }
    pub(crate) fn device_id(&self) -> DeviceId {
        self.data.1.device_id()
    }

    /// Creates a sendable device instance to allow for data loading in a background thread.
    pub fn create_background_loader(&self) -> Result<DeviceLoader<B>, B::Error> {
        let instance = self.internal(|d| B::create_background_loader(d))?;
        Ok(DeviceLoader::new(instance))
    }

    pub fn load<T: DeviceLoad<B>>(&self, data: &T) -> Result<Stored<B, T::LoadAs>, B::Error> {
        self.load_array(std::slice::from_ref(data))
    }

    pub fn load_array<T: DeviceLoad<B>>(
        &self,
        data: &[T],
    ) -> Result<Stored<B, T::LoadAs>, B::Error> {
        let options = B::default_load_options();
        self.internal(|d| T::load(d, data, options))
    }

    /// Loads data with custom options. Often these options will be the usage types (ex. `UsageFlags::RENDER_READ`).
    pub fn load_using<T: DeviceLoad<B>, Opts: BackendLoadOptions<B>>(
        &self,
        data: &T,
        usage: Opts,
    ) -> Result<Stored<B, T::LoadAs>, B::Error> {
        self.load_array_using(std::slice::from_ref(data), usage)
    }

    /// Loads array data with custom options. Often these options will be the usage types (ex. `UsageFlags::RENDER_READ`).
    pub fn load_array_using<T: DeviceLoad<B>, Opts: BackendLoadOptions<B>>(
        &self,
        data: &[T],
        usage: Opts,
    ) -> Result<Stored<B, T::LoadAs>, B::Error> {
        let usage = usage.create_load_options()?;
        self.internal(|d| T::load(d, data, usage))
    }

    pub unsafe fn download<T: DeviceDownload<B>>(
        &self,
        data: &Stored<B, T>,
    ) -> Result<T::DownloadAs, B::Error> {
        self.internal(|d| unsafe { T::download(d, data) })
    }

    pub fn flush_deletes(&self) -> Result<(), B::Error> {
        self.internal(|d| d.flush_deletes())
    }

    pub fn internal<T>(&self, action: impl FnOnce(&mut B::DeviceInstance) -> T) -> T {
        let mut data = self.data.0.borrow_mut();
        action(&mut data)
    }

    pub fn try_internal<T>(&self, action: impl FnOnce(Option<&mut B::DeviceInstance>) -> T) -> T {
        match self.data.0.try_borrow_mut() {
            Ok(mut d) => action(Some(&mut d)),
            Err(_) => action(None),
        }
    }

    pub(crate) fn index_manager(&self) -> &IndexManager {
        &self.data.1
    }
}
impl<B: Backend> Clone for Device<B> {
    fn clone(&self) -> Self {
        Device {
            data: self.data.clone(),
        }
    }
}

pub struct DeviceLoader<B: Backend> {
    instance: RefCell<B::SendDeviceInstance>,
}
impl<B: Backend> DeviceLoader<B> {
    pub(crate) fn new(instance: B::SendDeviceInstance) -> Self {
        DeviceLoader {
            instance: RefCell::new(instance),
        }
    }

    pub fn load<T: DeviceLoad<B>>(&self, data: &T) -> Result<StoredSend<B, T::LoadAs>, B::Error> {
        self.load_array(std::slice::from_ref(data))
    }

    pub fn load_array<T: DeviceLoad<B>>(
        &self,
        data: &[T],
    ) -> Result<StoredSend<B, T::LoadAs>, B::Error> {
        let options = B::default_load_options();
        self.internal(|d| T::load_sendable(d, data, options))
    }

    /// Loads data with custom options. Often these options will be the usage types (ex. `UsageFlags::RENDER_READ`).
    pub fn load_using<T: DeviceLoad<B>, Opts: BackendLoadOptions<B>>(
        &self,
        data: &T,
        usage: Opts,
    ) -> Result<StoredSend<B, T::LoadAs>, B::Error> {
        self.load_array_using(std::slice::from_ref(data), usage)
    }

    /// Loads array data with custom options. Often these options will be the usage types (ex. `UsageFlags::RENDER_READ`).
    pub fn load_array_using<T: DeviceLoad<B>, Opts: BackendLoadOptions<B>>(
        &self,
        data: &[T],
        usage: Opts,
    ) -> Result<StoredSend<B, T::LoadAs>, B::Error> {
        let usage = usage.create_load_options()?;
        self.internal(|d| T::load_sendable(d, data, usage))
    }

    pub fn internal<T>(&self, action: impl FnOnce(&mut B::SendDeviceInstance) -> T) -> T {
        let mut data = self.instance.borrow_mut();
        action(&mut data)
    }
}
