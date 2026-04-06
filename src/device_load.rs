use crate::*;

pub trait DeviceLoad<B: Backend>: Sized {
    type LoadAs;

    fn load(
        d: &mut B::DeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, B::Error>;

    fn reload(
        d: &mut B::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), B::Error>;

    fn load_sendable(
        d: &mut B::SendDeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, B::Error>;
}

pub trait DeviceDownload<B: Backend>: Sized {
    type DownloadAs;

    unsafe fn download(
        d: &mut B::DeviceInstance,
        data: &Stored<B, Self>,
    ) -> Result<Self::DownloadAs, B::Error>;
}

impl<B: Backend> DeviceLoad<B> for u32 {
    type LoadAs = u32;

    fn load(
        d: &mut B::DeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, B::Error> {
        let result = d.load_buffer_data(BufferData::IndexBufferU32 { data }, options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        d.reload_buffer_data(BufferData::IndexBufferU32 { data }, dest.stored_data())
    }

    fn load_sendable(
        d: &mut B::SendDeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, B::Error> {
        let result = d.load_buffer_data(BufferData::IndexBufferU32 { data }, options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }
}
impl<B: Backend> DeviceDownload<B> for u32 {
    type DownloadAs = Vec<u32>;

    unsafe fn download(
        d: &mut B::DeviceInstance,
        data: &Stored<B, Self>,
    ) -> Result<Self::DownloadAs, B::Error> {
        unsafe { d.download_buffer::<u32>(data.stored_data()) }
    }
}
impl IndexAsType for u32 {
    type Value = u32;
}
// Triangle indices as sets of three u32 values, will be loaded as a u32 array.
impl<B: Backend> DeviceLoad<B> for [u32; 3] {
    type LoadAs = u32;

    fn load(
        d: &mut B::DeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, B::Error> {
        let len_u32 = data.len() * 3;
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u32, len_u32) };
        let result = d.load_buffer_data(BufferData::IndexBufferU32 { data }, options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        let len_u32 = data.len() * 3;
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u32, len_u32) };
        d.reload_buffer_data(BufferData::IndexBufferU32 { data }, dest.stored_data())
    }

    fn load_sendable(
        d: &mut B::SendDeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, B::Error> {
        let len_u32 = data.len() * 3;
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u32, len_u32) };
        let result = d.load_buffer_data(BufferData::IndexBufferU32 { data }, options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }
}

impl<B: Backend> DeviceLoad<B> for u16 {
    type LoadAs = u16;

    fn load(
        d: &mut B::DeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, B::Error> {
        let result = d.load_buffer_data(BufferData::IndexBufferU16 { data }, options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        d.reload_buffer_data(BufferData::IndexBufferU16 { data }, dest.stored_data())
    }

    fn load_sendable(
        d: &mut B::SendDeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, B::Error> {
        let result = d.load_buffer_data(BufferData::IndexBufferU16 { data }, options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }
}
impl<B: Backend> DeviceDownload<B> for u16 {
    type DownloadAs = Vec<u16>;

    unsafe fn download(
        d: &mut B::DeviceInstance,
        data: &Stored<B, Self>,
    ) -> Result<Self::DownloadAs, B::Error> {
        unsafe { d.download_buffer::<u16>(data.stored_data()) }
    }
}
impl IndexAsType for u16 {
    type Value = u16;
}
// Triangle indices as sets of three u16 values, will be loaded as a u16 array.
impl<B: Backend> DeviceLoad<B> for [u16; 3] {
    type LoadAs = u16;

    fn load(
        d: &mut B::DeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, B::Error> {
        let len_u16 = data.len() * 3;
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u16, len_u16) };
        let result = d.load_buffer_data(BufferData::IndexBufferU16 { data }, options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        let len_u16 = data.len() * 3;
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u16, len_u16) };
        d.reload_buffer_data(BufferData::IndexBufferU16 { data }, dest.stored_data())
    }

    fn load_sendable(
        d: &mut B::SendDeviceInstance,
        data: &[Self],
        options: B::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, B::Error> {
        let len_u16 = data.len() * 3;
        let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u16, len_u16) };
        let result = d.load_buffer_data(BufferData::IndexBufferU16 { data }, options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }
}
