use crate::*;

pub enum BufferData<'a> {
    IndexBufferU16 {
        data: &'a [u16],
    },
    IndexBufferU32 {
        data: &'a [u32],
    },
    VertexBuffer {
        data: &'a [u8],
        struct_layout: &'static StructLayout,
        orig_len: usize,
    },
    InstanceBuffer {
        data: &'a [u8],
        struct_layout: &'static StructLayout,
        orig_len: usize,
    },
    UniformBuffer {
        data: &'a [u8],
        struct_layout: &'static StructLayout,
        orig_len: usize,
    },
    StorageBuffer {
        data: &'a [u8],
        struct_layout: &'static StructLayout,
        orig_len: usize,
    },
}
impl<'a> BufferData<'a> {
    /// Returns the data as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            BufferData::IndexBufferU16 { data } => unsafe {
                std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 2)
            },
            BufferData::IndexBufferU32 { data } => unsafe {
                std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4)
            },
            BufferData::VertexBuffer { data, .. }
            | BufferData::InstanceBuffer { data, .. }
            | BufferData::UniformBuffer { data, .. }
            | BufferData::StorageBuffer { data, .. } => data,
        }
    }

    /// Returns the original length of the buffer.
    pub fn orig_len(&self) -> usize {
        match self {
            BufferData::IndexBufferU16 { data } => data.len(),
            BufferData::IndexBufferU32 { data } => data.len(),
            BufferData::VertexBuffer { orig_len, .. }
            | BufferData::InstanceBuffer { orig_len, .. }
            | BufferData::UniformBuffer { orig_len, .. }
            | BufferData::StorageBuffer { orig_len, .. } => *orig_len,
        }
    }

    pub fn is_index_buffer(&self) -> bool {
        match self {
            BufferData::IndexBufferU16 { .. } | BufferData::IndexBufferU32 { .. } => true,
            _ => false,
        }
    }
    pub fn is_vertex_buffer(&self) -> bool {
        match self {
            BufferData::VertexBuffer { .. } => true,
            _ => false,
        }
    }
    pub fn is_instance_buffer(&self) -> bool {
        match self {
            BufferData::InstanceBuffer { .. } => true,
            _ => false,
        }
    }
    pub fn is_uniform_buffer(&self) -> bool {
        match self {
            BufferData::UniformBuffer { .. } => true,
            _ => false,
        }
    }

    pub fn is_vertex_or_instance_buffer(&self) -> bool {
        match self {
            BufferData::VertexBuffer { .. } | BufferData::InstanceBuffer { .. } => true,
            _ => false,
        }
    }
}

pub enum TextureData<'a, V> {
    AllocTexture1d([u32; 1]),
    Texture1d(TextureRef<'a, V, 1>),
    AllocTexture1dArray([u32; 1], usize),
    Texture1dArray(TextureRefArray<'a, V, 1>),

    AllocTexture2d([u32; 2]),
    Texture2d(TextureRef<'a, V, 2>),
    AllocTexture2dArray([u32; 2], usize),
    Texture2dArray(TextureRefArray<'a, V, 2>),

    AllocTexture3d([u32; 3]),
    Texture3d(TextureRef<'a, V, 3>),

    AllocTextureCube([u32; 2]),
    TextureCube(TextureCubeRef<'a, V>),
    //TextureCubeArray(TextureCubeRefArray<'a, V>),
}
impl<'a, V> TextureData<'a, V> {
    pub fn dim(&self) -> [u32; 3] {
        match self {
            TextureData::AllocTexture1d(dim) => [dim[0], 1, 1],
            TextureData::Texture1d(t) => [t.dim()[0], 1, 1],
            TextureData::AllocTexture1dArray(dim, _) => [dim[0], 1, 1],
            TextureData::Texture1dArray(a) => [a.dim()[0], 1, 1],
            TextureData::AllocTexture2d(dim) => [dim[0], dim[1], 1],
            TextureData::Texture2d(t) => [t.dim()[0], t.dim()[1], 1],
            TextureData::AllocTexture2dArray(dim, _) => [dim[0], dim[1], 1],
            TextureData::Texture2dArray(a) => [a.dim()[0], a.dim()[1], 1],
            TextureData::AllocTexture3d(dim) => [dim[0], dim[1], dim[2]],
            TextureData::Texture3d(t) => [t.dim()[0], t.dim()[1], t.dim()[2]],
            TextureData::AllocTextureCube(dim) => [dim[0], dim[1], 6],
            TextureData::TextureCube(c) => {
                let dim = c.face_dim();
                [dim[0], dim[1], 6]
            }
        }
    }

    pub fn is_alloc(&self) -> bool {
        match self {
            TextureData::AllocTexture1d(..)
            | TextureData::AllocTexture1dArray(..)
            | TextureData::AllocTexture2d(..)
            | TextureData::AllocTexture2dArray(..)
            | TextureData::AllocTexture3d(..)
            | TextureData::AllocTextureCube(..) => true,
            TextureData::Texture1d(..)
            | TextureData::Texture1dArray(..)
            | TextureData::Texture2d(..)
            | TextureData::Texture2dArray(..)
            | TextureData::Texture3d(..)
            | TextureData::TextureCube(..) => false,
        }
    }

    pub fn as_image_rows(&self) -> Vec<Vec<&'a [V]>> {
        match self {
            TextureData::AllocTexture1d(..) => Vec::new(),
            TextureData::Texture1d(t) => vec![t.rows().collect()],
            TextureData::AllocTexture1dArray(..) => Vec::new(),
            TextureData::Texture1dArray(a) => {
                a.textures().iter().map(|t| t.rows().collect()).collect()
            }
            TextureData::AllocTexture2d(..) => Vec::new(),
            TextureData::Texture2d(t) => vec![t.rows().collect()],
            TextureData::AllocTexture2dArray(..) => Vec::new(),
            TextureData::Texture2dArray(a) => {
                a.textures().iter().map(|t| t.rows().collect()).collect()
            }
            TextureData::AllocTexture3d(..) => Vec::new(),
            TextureData::Texture3d(t) => vec![t.rows().collect()],
            TextureData::AllocTextureCube(..) => Vec::new(),
            TextureData::TextureCube(c) => c
                .as_texture_ref_array()
                .textures()
                .iter()
                .map(|t| t.rows().collect())
                .collect(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Array<T, const N: usize> {
    pub data: [T; N],
}
impl<T: Default, const N: usize> Default for Array<T, N> {
    fn default() -> Self {
        Self {
            data: [(); N].map(|_| T::default()),
        }
    }
}
impl<V: DeviceValueTypeOrStruct, const N: usize> DeviceValueTypeOrStruct for Array<V, N> {
    fn add_data_to_struct_layout<'a, T>(
        layout: &mut StructLayoutHelper<'a, T>,
        field: impl Fn(&T) -> &Self,
        name: &str,
    ) {
        for i in 0..N {
            let field_name = format!("{}<{}>[{}]", name, N, i);
            layout.add_field(|s| &field(s).data[i], &field_name);
        }
    }
}
