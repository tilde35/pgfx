#[macro_export]
macro_rules! impl_data_struct {
    ($t:ty as $initial_value:expr, $($f:ident),* $(,)?) => {
        impl $crate::DataStruct for $t {
            fn device_struct_layout() -> &'static $crate::StructLayout {
                use std::sync::LazyLock;
                static LAYOUT: LazyLock<$crate::StructLayout> = LazyLock::new(|| {
                    let v: $t = $initial_value;
                    $crate::StructLayout::from_value(&v, stringify!($t))
                        $(.add_field(|v| &v.$f, stringify!($f)))*
                        .build()
                });
                &LAYOUT
            }
        }
        impl $crate::DeviceValueTypeOrStruct for $t {
            fn add_data_to_struct_layout<'a, T>(
                layout: &mut $crate::StructLayoutHelper<'a, T>,
                field: impl Fn(&T) -> &Self,
                name: &str,
            ) {
                layout.add_struct_field(field, name);
            }
        }
    };
    ($t:ty, $($f:ident),* $(,)?) => {
        impl $crate::DataStruct for $t {
            fn device_struct_layout() -> &'static $crate::StructLayout {
                use std::sync::LazyLock;
                static LAYOUT: LazyLock<$crate::StructLayout> = LazyLock::new(|| {
                    let v: $t = <$t as Default>::default();
                    $crate::StructLayout::from_value(&v, stringify!($t))
                        $(.add_field(|v| &v.$f, stringify!($f)))*
                        .build()
                });
                &LAYOUT
            }
        }
        impl $crate::DeviceValueTypeOrStruct for $t {
            fn add_data_to_struct_layout<'a, T>(
                layout: &mut $crate::StructLayoutHelper<'a, T>,
                field: impl Fn(&T) -> &Self,
                name: &str,
            ) {
                layout.add_struct_field(field, name);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vertex {
    ($name:ty) => {
        impl<B: $crate::Backend> $crate::DeviceDownload<B> for $name {
            type DownloadAs = Vec<$name>;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &$crate::Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                use $crate::BackendDeviceInstance;
                unsafe { d.download_buffer(data.stored_data()) }
            }
        }
        impl $crate::IndexAsType for $name {
            type Value = $name;
        }
        impl<B: $crate::Backend> $crate::DeviceLoad<B> for $name {
            type LoadAs = $name;

            fn load(
                d: &mut B::DeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::Stored<B, Self::LoadAs>, B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::VertexBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }

            fn reload(
                d: &mut B::DeviceInstance,
                data: &[Self],
                dest: &$crate::Stored<B, Self::LoadAs>,
            ) -> Result<(), B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                d.reload_buffer_data(
                    $crate::BufferData::VertexBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    dest.stored_data(),
                )
            }

            fn load_sendable(
                d: &mut B::SendDeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::StoredSend<B, Self::LoadAs>, B::Error> {
                use $crate::BackendSendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::VertexBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_instance {
    ($name:ty) => {
        impl<B: $crate::Backend> $crate::DeviceDownload<B> for $name {
            type DownloadAs = Vec<$name>;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &$crate::Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                use $crate::BackendDeviceInstance;
                unsafe { d.download_buffer(data.stored_data()) }
            }
        }
        impl $crate::IndexAsType for $name {
            type Value = $name;
        }
        impl<B: $crate::Backend> $crate::DeviceLoad<B> for $name {
            type LoadAs = $name;

            fn load(
                d: &mut B::DeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::Stored<B, Self::LoadAs>, B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::InstanceBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }

            fn reload(
                d: &mut B::DeviceInstance,
                data: &[Self],
                dest: &$crate::Stored<B, Self::LoadAs>,
            ) -> Result<(), B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                d.reload_buffer_data(
                    $crate::BufferData::InstanceBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    dest.stored_data(),
                )
            }

            fn load_sendable(
                d: &mut B::SendDeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::StoredSend<B, Self::LoadAs>, B::Error> {
                use $crate::BackendSendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::InstanceBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_uniform {
    ($name:ty) => {
        impl<B: $crate::Backend> $crate::DeviceDownload<B> for $name {
            type DownloadAs = $name;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &$crate::Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                use $crate::BackendDeviceInstance;
                let mut data = unsafe { d.download_buffer(data.stored_data()) }?;
                Ok(data.remove(0))
            }
        }
        impl $crate::IndexAsType for $name {
            type Value = $name;
        }
        impl<B: $crate::Backend> $crate::DeviceLoad<B> for $name {
            type LoadAs = $name;

            fn load(
                d: &mut B::DeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::Stored<B, Self::LoadAs>, B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::UniformBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }

            fn reload(
                d: &mut B::DeviceInstance,
                data: &[Self],
                dest: &$crate::Stored<B, Self::LoadAs>,
            ) -> Result<(), B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                d.reload_buffer_data(
                    $crate::BufferData::UniformBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    dest.stored_data(),
                )
            }

            fn load_sendable(
                d: &mut B::SendDeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::StoredSend<B, Self::LoadAs>, B::Error> {
                use $crate::BackendSendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::UniformBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_storage {
    ($name:ty) => {
        impl<B: $crate::Backend> $crate::DeviceDownload<B> for $name {
            type DownloadAs = $name;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &$crate::Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                use $crate::BackendDeviceInstance;
                let mut data = unsafe { d.download_buffer(data.stored_data()) }?;
                Ok(data.remove(0))
            }
        }
        impl $crate::IndexAsType for $name {
            type Value = $name;
        }
        impl<B: $crate::Backend> $crate::DeviceLoad<B> for $name {
            type LoadAs = $name;

            fn load(
                d: &mut B::DeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::Stored<B, Self::LoadAs>, B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::StorageBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }

            fn reload(
                d: &mut B::DeviceInstance,
                data: &[Self],
                dest: &$crate::Stored<B, Self::LoadAs>,
            ) -> Result<(), B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                d.reload_buffer_data(
                    $crate::BufferData::StorageBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    dest.stored_data(),
                )
            }

            fn load_sendable(
                d: &mut B::SendDeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::StoredSend<B, Self::LoadAs>, B::Error> {
                use $crate::BackendSendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::StorageBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }
        }
    };
}

/// Creates a struct that can be loaded into a device and be used as a vertex array.
///
/// This macro provides the following:
/// - Ensures that the struct is `repr(C)` and `Clone + Copy + Default + Debug`.
/// - Implements `DataStruct` to define the field layout.
/// - Implements `DeviceData` for the struct as a vertex array value.
///
/// Sample usage:
/// ```ignore
/// vertex_struct!(
///     pub struct MyVertex {
///         pub position: [f32; 3],
///         pub color: [f32; 3],
///     }
/// );
/// ```
#[macro_export]
macro_rules! vertex_struct {
    (
        $struct_vis:vis struct $name:ident {
            $($field_vis:vis $field:ident : $typ:ty),* $(,)*
        }
    ) => {
        #[repr(C)]
        #[derive(Clone, Copy, Default, Debug)]
        $struct_vis struct $name {
            $($field_vis $field: $typ),*
        }
        impl $crate::DataStruct for $name {
            fn device_struct_layout() -> &'static $crate::StructLayout {
                use std::sync::LazyLock;
                static LAYOUT: LazyLock<$crate::StructLayout> = LazyLock::new(|| {
                    let v: $name = Default::default();
                    $crate::StructLayout::from_value(&v, stringify!($t))
                        $(.add_field(|v| &v.$field, stringify!($field)))*
                        .build()
                });
                &LAYOUT
            }
        }
        impl $crate::DeviceValueTypeOrStruct for $name {
            fn add_data_to_struct_layout<'a, T>(
                layout: &mut $crate::StructLayoutHelper<'a, T>,
                field: impl Fn(&T) -> &Self,
                name: &str,
            ) {
                layout.add_struct_field(field, name);
            }
        }
        impl<B: $crate::Backend> $crate::DeviceDownload<B> for $name {
            type DownloadAs = Vec<$name>;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &$crate::Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                use $crate::BackendDeviceInstance;
                unsafe { d.download_buffer(data.stored_data()) }
            }
        }
        impl $crate::IndexAsType for $name {
            type Value = $name;
        }
        impl<B: $crate::Backend> $crate::DeviceLoad<B> for $name {
            type LoadAs = $name;

            fn load(
                d: &mut B::DeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::Stored<B, Self::LoadAs>, B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::VertexBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }

            fn reload(
                d: &mut B::DeviceInstance,
                data: &[Self],
                dest: &$crate::Stored<B, Self::LoadAs>,
            ) -> Result<(), B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                d.reload_buffer_data(
                    $crate::BufferData::VertexBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    dest.stored_data(),
                )
            }

            fn load_sendable(
                d: &mut B::SendDeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::StoredSend<B, Self::LoadAs>, B::Error> {
                use $crate::BackendSendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::VertexBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }
        }
    };
}

/// Creates a struct that can be loaded into a device and be used as a instance array.
///
/// This macro provides the following:
/// - Ensures that the struct is `repr(C)` and `Clone + Copy + Default + Debug`.
/// - Implements `DataStruct` to define the field layout.
/// - Implements `DeviceData` for the struct as a instance array value.
///
/// Sample usage:
/// ```ignore
/// instance_struct!(
///     pub struct MyVertex {
///         pub position: [f32; 3],
///         pub color: [f32; 3],
///     }
/// );
/// ```
#[macro_export]
macro_rules! instance_struct {
    (
        $struct_vis:vis struct $name:ident {
            $($field_vis:vis $field:ident : $typ:ty),* $(,)*
        }
    ) => {
        #[repr(C)]
        #[derive(Clone, Copy, Default, Debug)]
        $struct_vis struct $name {
            $($field_vis $field: $typ),*
        }
        impl $crate::DataStruct for $name {
            fn device_struct_layout() -> &'static $crate::StructLayout {
                use std::sync::LazyLock;
                static LAYOUT: LazyLock<$crate::StructLayout> = LazyLock::new(|| {
                    let v: $name = Default::default();
                    $crate::StructLayout::from_value(&v, stringify!($t))
                        $(.add_field(|v| &v.$field, stringify!($field)))*
                        .build()
                });
                &LAYOUT
            }
        }
        impl $crate::DeviceValueTypeOrStruct for $name {
            fn add_data_to_struct_layout<'a, T>(
                layout: &mut $crate::StructLayoutHelper<'a, T>,
                field: impl Fn(&T) -> &Self,
                name: &str,
            ) {
                layout.add_struct_field(field, name);
            }
        }
        impl<B: $crate::Backend> $crate::DeviceDownload<B> for $name {
            type DownloadAs = Vec<$name>;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &$crate::Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                use $crate::BackendDeviceInstance;
                unsafe { d.download_buffer(data.stored_data()) }
            }
        }
        impl $crate::IndexAsType for $name {
            type Value = $name;
        }
        impl<B: $crate::Backend> $crate::DeviceLoad<B> for $name {
            type LoadAs = $name;

            fn load(
                d: &mut B::DeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::Stored<B, Self::LoadAs>, B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::InstanceBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }

            fn reload(
                d: &mut B::DeviceInstance,
                data: &[Self],
                dest: &$crate::Stored<B, Self::LoadAs>,
            ) -> Result<(), B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                d.reload_buffer_data(
                    $crate::BufferData::InstanceBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    dest.stored_data(),
                )
            }

            fn load_sendable(
                d: &mut B::SendDeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::StoredSend<B, Self::LoadAs>, B::Error> {
                use $crate::BackendSendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::InstanceBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }
        }
    };
}

/// Creates a struct that can be loaded into a device and be used as a uniform buffer.
///
/// This macro provides the following:
/// - Ensures that the struct is `repr(C)` and `Clone + Copy + Default + Debug`.
/// - Implements `DataStruct` to define the field layout.
/// - Implements `DeviceData` for the struct as a uniform buffer value.
///
/// Sample usage:
/// ```ignore
/// uniform_struct!(
///     pub struct MyUniform {
///         pub light_color: [f32; 4],
///         pub view: [[f32; 4]; 4],
///         pub proj: [[f32; 4]; 4],
///     }
/// );
/// ```
#[macro_export]
macro_rules! uniform_struct {
    (
        $struct_vis:vis struct $name:ident {
            $($field_vis:vis $field:ident : $typ:ty),* $(,)*
        }
    ) => {
        #[repr(C)]
        #[derive(Clone, Copy, Default, Debug)]
        $struct_vis struct $name {
            $($field_vis $field: $typ),*
        }
        impl $crate::DataStruct for $name {
            fn device_struct_layout() -> &'static $crate::StructLayout {
                use std::sync::LazyLock;
                static LAYOUT: LazyLock<$crate::StructLayout> = LazyLock::new(|| {
                    let v: $name = Default::default();
                    $crate::StructLayout::from_value(&v, stringify!($t))
                        $(.add_field(|v| &v.$field, stringify!($field)))*
                        .build()
                });
                &LAYOUT
            }
        }
        impl $crate::DeviceValueTypeOrStruct for $name {
            fn add_data_to_struct_layout<'a, T>(
                layout: &mut $crate::StructLayoutHelper<'a, T>,
                field: impl Fn(&T) -> &Self,
                name: &str,
            ) {
                layout.add_struct_field(field, name);
            }
        }
        impl<B: $crate::Backend> $crate::DeviceDownload<B> for $name {
            type DownloadAs = $name;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &$crate::Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                use $crate::BackendDeviceInstance;
                let mut data = unsafe { d.download_buffer(data.stored_data()) }?;
                Ok(data.remove(0))
            }
        }
        impl $crate::IndexAsType for $name {
            type Value = $name;
        }
        impl<B: $crate::Backend> $crate::DeviceLoad<B> for $name {
            type LoadAs = $name;

            fn load(
                d: &mut B::DeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::Stored<B, Self::LoadAs>, B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::UniformBuffer {
                        data: bytes,
                        struct_layout,
                            orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }

            fn reload(
                d: &mut B::DeviceInstance,
                data: &[Self],
                dest: &$crate::Stored<B, Self::LoadAs>,
            ) -> Result<(), B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                d.reload_buffer_data(
                    $crate::BufferData::UniformBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    dest.stored_data(),
                )
            }

            fn load_sendable(
                d: &mut B::SendDeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::StoredSend<B, Self::LoadAs>, B::Error> {
                use $crate::BackendSendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::UniformBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }
        }
    };
}

/// Creates a struct that can be loaded into a device and be used as a storage buffer.
///
/// This macro provides the following:
/// - Ensures that the struct is `repr(C)` and `Clone + Copy + Default + Debug`.
/// - Implements `DataStruct` to define the field layout.
/// - Implements `DeviceData` for the struct as a storage buffer value.
///
/// Sample usage:
/// ```ignore
/// storage_struct!(
///     pub struct MyStorage {
///         pub light_color: [f32; 4],
///         pub view: [[f32; 4]; 4],
///         pub proj: [[f32; 4]; 4],
///     }
/// );
/// ```
#[macro_export]
macro_rules! storage_struct {
    (
        $struct_vis:vis struct $name:ident {
            $($field_vis:vis $field:ident : $typ:ty),* $(,)*
        }
    ) => {
        #[repr(C)]
        #[derive(Clone, Copy, Default, Debug)]
        $struct_vis struct $name {
            $($field_vis $field: $typ),*
        }
        impl $crate::DataStruct for $name {
            fn device_struct_layout() -> &'static $crate::StructLayout {
                use std::sync::LazyLock;
                static LAYOUT: LazyLock<$crate::StructLayout> = LazyLock::new(|| {
                    let v: $name = Default::default();
                    $crate::StructLayout::from_value(&v, stringify!($t))
                        $(.add_field(|v| &v.$field, stringify!($field)))*
                        .build()
                });
                &LAYOUT
            }
        }
        impl $crate::DeviceValueTypeOrStruct for $name {
            fn add_data_to_struct_layout<'a, T>(
                layout: &mut $crate::StructLayoutHelper<'a, T>,
                field: impl Fn(&T) -> &Self,
                name: &str,
            ) {
                layout.add_struct_field(field, name);
            }
        }
        impl<B: $crate::Backend> $crate::DeviceDownload<B> for $name {
            type DownloadAs = $name;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &$crate::Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                use $crate::BackendDeviceInstance;
                let mut data = unsafe { d.download_buffer(data.stored_data()) }?;
                Ok(data.remove(0))
            }
        }
        impl $crate::IndexAsType for $name {
            type Value = $name;
        }
        impl<B: $crate::Backend> $crate::DeviceLoad<B> for $name {
            type LoadAs = $name;

            fn load(
                d: &mut B::DeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::Stored<B, Self::LoadAs>, B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::StorageBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }

            fn reload(
                d: &mut B::DeviceInstance,
                data: &[Self],
                dest: &$crate::Stored<B, Self::LoadAs>,
            ) -> Result<(), B::Error> {
                use $crate::BackendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                d.reload_buffer_data(
                    $crate::BufferData::StorageBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    dest.stored_data(),
                )
            }

            fn load_sendable(
                d: &mut B::SendDeviceInstance,
                data: &[Self],
                options: B::LoadOptions,
            ) -> Result<$crate::StoredSend<B, Self::LoadAs>, B::Error> {
                use $crate::BackendSendDeviceInstance;
                let struct_layout = <$name as $crate::DataStruct>::device_struct_layout();
                let bytes = unsafe {
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        data.len() * std::mem::size_of::<Self>(),
                    )
                };
                let result = d.load_buffer_data(
                    $crate::BufferData::StorageBuffer {
                        data: bytes,
                        struct_layout,
                        orig_len: data.len(),
                    },
                    options,
                )?;
                unsafe { Ok(result.unchecked_cast()) }
            }
        }
    };
}

#[macro_export]
macro_rules! data_value_mapping {
    ($($t:ty as $other:ty),* $(,)?) => {
        $(
            impl $crate::DeviceValueType for $t {
                fn device_value_type() -> $crate::ValueType {
                    const {
                        // Size must be the same
                        assert!(std::mem::size_of::<$t>() == std::mem::size_of::<$other>());
                        // Alignment can be more restrictive (larger), but not less restrictive
                        assert!(std::mem::align_of::<$t>() >= std::mem::align_of::<$other>());
                    }
                    <$other as $crate::DeviceValueType>::device_value_type()
                }
            }
        )*
    };
}
