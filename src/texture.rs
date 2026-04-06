use crate::*;

#[derive(Clone)]
pub struct TextureAlloc<V, const N: usize = 2> {
    dim: [u32; N],
    _data: std::marker::PhantomData<V>,
}
impl<V, const N: usize> TextureAlloc<V, N> {
    pub const fn new(dim: [u32; N]) -> Self {
        Self {
            dim,
            _data: std::marker::PhantomData,
        }
    }

    pub const fn dim(&self) -> [u32; N] {
        self.dim
    }
}
impl<const N: usize> TextureAlloc<UNorm8, N> {
    pub const fn new_unorm8(dim: [u32; N]) -> Self {
        Self::new(dim)
    }
}
impl<const N: usize> TextureAlloc<f32, N> {
    pub const fn new_f32(dim: [u32; N]) -> Self {
        Self::new(dim)
    }
}
impl<const N: usize> TextureAlloc<Depth, N> {
    pub const fn new_depth(dim: [u32; N]) -> Self {
        Self::new(dim)
    }
}
impl<const N: usize> TextureAlloc<[UNorm8; 4], N> {
    pub const fn new_lrgba(dim: [u32; N]) -> Self {
        Self::new(dim)
    }
}
impl<const N: usize> TextureAlloc<Srgba, N> {
    pub const fn new_srgba(dim: [u32; N]) -> Self {
        Self::new(dim)
    }
}

#[derive(Clone)]
pub struct Texture<V, const N: usize = 2> {
    dim: [u32; N],
    data: Vec<V>,
}
impl<V, const N: usize> Texture<V, N> {
    pub fn new(dim: [u32; N]) -> Self
    where
        V: Default,
    {
        let len = dim.iter().product::<u32>() as usize;
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(V::default());
        }
        Self { dim, data }
    }
    pub fn from_parts(dim: [u32; N], data: Vec<V>) -> Self {
        let len = dim.iter().product::<u32>() as usize;
        assert!(data.len() == len, "Data length does not match dimensions");
        Self { dim, data }
    }

    pub fn dim(&self) -> [u32; N] {
        self.dim
    }

    pub fn data(&self) -> &[V] {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut [V] {
        &mut self.data
    }

    pub fn index_of(&self, pos: [i32; N]) -> Option<usize> {
        let mut index = 0;
        let mut stride = 1;
        for i in 0..N {
            let p = pos[i];
            if p < 0 || p as u32 >= self.dim[i] {
                return None;
            }
            index += p as usize * stride;
            stride *= self.dim[i] as usize;
        }
        Some(index)
    }
    pub fn unchecked_index_of(&self, pos: [u32; N]) -> usize {
        let mut index = 0;
        let mut stride = 1;
        for i in 0..N {
            index += pos[i] as usize * stride;
            stride *= self.dim[i] as usize;
        }
        index
    }

    pub fn get(&self, pos: [u32; N]) -> &V {
        let idx = self
            .index_of(pos.map(|v| v as i32))
            .expect("index in bounds");
        &self.data[idx]
    }
    pub fn try_get(&self, pos: [i32; N]) -> Option<&V> {
        self.index_of(pos).map(|idx| &self.data[idx])
    }
    pub fn set(&mut self, pos: [u32; N], value: V) {
        let idx = self
            .index_of(pos.map(|v| v as i32))
            .expect("index in bounds");
        self.data[idx] = value;
    }
    pub fn try_set(&mut self, pos: [i32; N], value: V) {
        if let Some(idx) = self.index_of(pos) {
            self.data[idx] = value;
        }
    }

    pub fn sample_nearest(&self, uv: [f32; N]) -> &V {
        self.as_texture_ref().sample_nearest(uv)
    }

    pub fn into_data(self) -> Vec<V> {
        self.data
    }

    pub fn as_texture_ref<'r>(&'r self) -> TextureRef<'r, V, N> {
        TextureRef {
            dim: self.dim,
            data: &self.data,
        }
    }
    pub fn as_texture_mut<'r>(&'r mut self) -> TextureMut<'r, V, N> {
        TextureMut {
            dim: self.dim,
            data: &mut self.data,
        }
    }
}
impl<V> Texture<V, 2> {
    pub fn flip_x(&mut self) {
        self.as_texture_mut().flip_x();
    }
    pub fn flip_y(&mut self) {
        self.as_texture_mut().flip_y();
    }
}

pub struct TextureRef<'a, V, const N: usize = 2> {
    dim: [u32; N],
    data: &'a [V],
}
impl<'a, V, const N: usize> TextureRef<'a, V, N> {
    pub fn new(dim: [u32; N], data: &'a [V]) -> Self {
        let len = dim.iter().product::<u32>() as usize;
        assert!(data.len() == len, "Data length does not match dimensions");
        Self { dim, data }
    }

    pub fn dim(&self) -> [u32; N] {
        self.dim
    }

    pub fn data(&self) -> &[V] {
        self.data
    }

    pub fn index_of(&self, pos: [i32; N]) -> Option<usize> {
        let mut index = 0;
        let mut stride = 1;
        for i in 0..N {
            let p = pos[i];
            if p < 0 || p as u32 >= self.dim[i] {
                return None;
            }
            index += p as usize * stride;
            stride *= self.dim[i] as usize;
        }
        Some(index)
    }
    pub fn unchecked_index_of(&self, pos: [u32; N]) -> usize {
        let mut index = 0;
        let mut stride = 1;
        for i in 0..N {
            index += pos[i] as usize * stride;
            stride *= self.dim[i] as usize;
        }
        index
    }

    pub fn get(&self, pos: [u32; N]) -> &'a V {
        let idx = self
            .index_of(pos.map(|v| v as i32))
            .expect("index in bounds");
        &self.data[idx]
    }
    pub fn try_get(&self, pos: [i32; N]) -> Option<&'a V> {
        self.index_of(pos).map(|idx| &self.data[idx])
    }

    pub fn sample_nearest(&self, uv: [f32; N]) -> &'a V {
        let mut pos = [0u32; N];

        for i in 0..N {
            let u = (uv[i] * self.dim[i] as f32) as i32;
            pos[i] = (u.clamp(0, self.dim[i] as i32 - 1)) as u32;
        }

        self.get(pos)
    }

    pub unsafe fn transmute<U>(self) -> TextureRef<'a, U, N> {
        const {
            assert!(std::mem::size_of::<V>() == std::mem::size_of::<U>());
            assert!(std::mem::align_of::<V>() == std::mem::align_of::<U>());
        }
        TextureRef {
            dim: self.dim,
            data: unsafe { std::mem::transmute(self.data) },
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &'a [V]> {
        let row_size = self.dim[0] as usize;
        self.data.chunks(row_size)
    }

    pub fn to_texture(&self) -> Texture<V, N>
    where
        V: Clone,
    {
        Texture {
            dim: self.dim,
            data: self.data.to_vec(),
        }
    }

    // Note: Simplifies the below macros
    fn as_texture_ref(&self) -> Self {
        Self {
            dim: self.dim,
            data: self.data,
        }
    }
}
impl<'a, V, const N: usize> Clone for TextureRef<'a, V, N> {
    fn clone(&self) -> Self {
        Self {
            dim: self.dim,
            data: self.data,
        }
    }
}

pub struct TextureMut<'a, V, const N: usize = 2> {
    dim: [u32; N],
    data: &'a mut [V],
}
impl<'a, V, const N: usize> TextureMut<'a, V, N> {
    pub fn new(dim: [u32; N], data: &'a mut [V]) -> Self {
        let len = dim.iter().product::<u32>() as usize;
        assert!(data.len() == len, "Data length does not match dimensions");
        Self { dim, data }
    }

    pub fn dim(&self) -> [u32; N] {
        self.dim
    }

    pub fn data(&mut self) -> &mut [V] {
        self.data
    }

    pub fn index_of(&self, pos: [i32; N]) -> Option<usize> {
        let mut index = 0;
        let mut stride = 1;
        for i in 0..N {
            let p = pos[i];
            if p < 0 || p as u32 >= self.dim[i] {
                return None;
            }
            index += p as usize * stride;
            stride *= self.dim[i] as usize;
        }
        Some(index)
    }
    pub fn unchecked_index_of(&self, pos: [u32; N]) -> usize {
        let mut index = 0;
        let mut stride = 1;
        for i in 0..N {
            index += pos[i] as usize * stride;
            stride *= self.dim[i] as usize;
        }
        index
    }

    pub fn get(&self, pos: [u32; N]) -> &V {
        let idx = self
            .index_of(pos.map(|v| v as i32))
            .expect("index in bounds");
        &self.data[idx]
    }
    pub fn try_get(&self, pos: [i32; N]) -> Option<&V> {
        self.index_of(pos).map(|idx| &self.data[idx])
    }
    pub fn set(&mut self, pos: [u32; N], value: V) {
        let idx = self
            .index_of(pos.map(|v| v as i32))
            .expect("index in bounds");
        self.data[idx] = value;
    }
    pub fn try_set(&mut self, pos: [i32; N], value: V) -> bool {
        if let Some(idx) = self.index_of(pos) {
            self.data[idx] = value;
            true
        } else {
            false
        }
    }

    pub fn sample_nearest(&self, uv: [f32; N]) -> &V {
        self.as_texture_ref().sample_nearest(uv)
    }

    pub fn to_texture(&self) -> Texture<V, N>
    where
        V: Clone,
    {
        Texture {
            dim: self.dim,
            data: self.data.to_vec(),
        }
    }

    pub fn as_texture_ref(&self) -> TextureRef<'_, V, N> {
        TextureRef {
            dim: self.dim,
            data: self.data,
        }
    }
}
impl<'a, V> TextureMut<'a, V, 2> {
    pub fn flip_x(&mut self) {
        let width = self.dim[0] as usize;
        let height = self.dim[1] as usize;
        for y in 0..height {
            for x in 0..(width / 2) {
                let left_idx = y * width + x;
                let right_idx = y * width + (width - 1 - x);
                self.data.swap(left_idx, right_idx);
            }
        }
    }

    pub fn flip_y(&mut self) {
        let width = self.dim[0] as usize;
        let height = self.dim[1] as usize;
        for y in 0..(height / 2) {
            for x in 0..width {
                let top_idx = y * width + x;
                let bottom_idx = (height - 1 - y) * width + x;
                self.data.swap(top_idx, bottom_idx);
            }
        }
    }
}

#[derive(Clone)]
pub struct TextureArrayAlloc<V, const N: usize = 2> {
    dim: [u32; N],
    len: usize,
    _data: std::marker::PhantomData<V>,
}
impl<V, const N: usize> TextureArrayAlloc<V, N> {
    pub const fn new(dim: [u32; N], len: usize) -> Self {
        Self {
            dim,
            len,
            _data: std::marker::PhantomData,
        }
    }

    pub const fn dim(&self) -> [u32; N] {
        self.dim
    }

    pub const fn len(&self) -> usize {
        self.len
    }
}
impl<const N: usize> TextureArrayAlloc<UNorm8, N> {
    pub const fn new_unorm8(dim: [u32; N], len: usize) -> Self {
        Self::new(dim, len)
    }
}
impl<const N: usize> TextureArrayAlloc<f32, N> {
    pub const fn new_f32(dim: [u32; N], len: usize) -> Self {
        Self::new(dim, len)
    }
}
impl<const N: usize> TextureArrayAlloc<Depth, N> {
    pub const fn new_depth(dim: [u32; N], len: usize) -> Self {
        Self::new(dim, len)
    }
}
impl<const N: usize> TextureArrayAlloc<[UNorm8; 4], N> {
    pub const fn new_lrgba(dim: [u32; N], len: usize) -> Self {
        Self::new(dim, len)
    }
}
impl<const N: usize> TextureArrayAlloc<Srgba, N> {
    pub const fn new_srgba(dim: [u32; N], len: usize) -> Self {
        Self::new(dim, len)
    }
}

#[derive(Clone)]
pub struct TextureArray<V, const N: usize = 2> {
    textures: Vec<Texture<V, N>>,
}
impl<V, const N: usize> TextureArray<V, N> {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            textures: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, texture: Texture<V, N>) {
        if let Some(first) = self.textures.first() {
            assert_eq!(
                first.dim, texture.dim,
                "All textures must have the same dimensions"
            );
        }
        self.textures.push(texture);
    }

    pub fn dim(&self) -> [u32; N] {
        if let Some(first) = self.textures.first() {
            first.dim
        } else {
            [0; N]
        }
    }

    pub fn textures(&self) -> &[Texture<V, N>] {
        &self.textures
    }

    pub fn get(&self, index: usize) -> &Texture<V, N> {
        &self.textures[index]
    }
    pub fn try_get(&self, index: usize) -> Option<&Texture<V, N>> {
        self.textures.get(index)
    }

    pub fn len(&self) -> usize {
        self.textures.len()
    }

    pub fn as_texture_ref_array(&self) -> TextureRefArray<'_, V, N> {
        TextureRefArray {
            textures: self.textures.iter().map(|t| t.as_texture_ref()).collect(),
        }
    }
}
impl<V> TextureArray<V, 2> {
    pub fn flip_x(&mut self) {
        for texture in &mut self.textures {
            texture.as_texture_mut().flip_x();
        }
    }

    pub fn flip_y(&mut self) {
        for texture in &mut self.textures {
            texture.as_texture_mut().flip_y();
        }
    }
}

pub struct TextureRefArray<'a, V, const N: usize = 2> {
    textures: Vec<TextureRef<'a, V, N>>,
}
impl<'a, V, const N: usize> TextureRefArray<'a, V, N> {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            textures: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, texture: TextureRef<'a, V, N>) {
        if let Some(first) = self.textures.first() {
            assert_eq!(
                first.dim, texture.dim,
                "All textures must have the same dimensions"
            );
        }
        self.textures.push(texture);
    }

    pub fn dim(&self) -> [u32; N] {
        if let Some(first) = self.textures.first() {
            first.dim
        } else {
            [0; N]
        }
    }

    pub fn textures(&self) -> &[TextureRef<'a, V, N>] {
        &self.textures
    }

    pub fn get(&self, index: usize) -> &TextureRef<'a, V, N> {
        &self.textures[index]
    }
    pub fn try_get(&self, index: usize) -> Option<&TextureRef<'a, V, N>> {
        self.textures.get(index)
    }

    pub fn len(&self) -> usize {
        self.textures.len()
    }

    pub fn to_texture_array(&self) -> TextureArray<V, N>
    where
        V: Clone,
    {
        TextureArray {
            textures: self.textures.iter().map(|t| t.to_texture()).collect(),
        }
    }

    // Note: Simplifies the below macros
    fn as_texture_ref_array(&self) -> Self {
        Self {
            textures: self.textures.clone(),
        }
    }
}
impl<'a, V, const N: usize> Clone for TextureRefArray<'a, V, N> {
    fn clone(&self) -> Self {
        Self {
            textures: self.textures.clone(),
        }
    }
}

#[derive(Clone)]
pub struct TextureCubeAlloc<V> {
    dim: [u32; 2],
    _data: std::marker::PhantomData<V>,
}
impl<V> TextureCubeAlloc<V> {
    pub const fn new(dim: [u32; 2]) -> Self {
        Self {
            dim,
            _data: std::marker::PhantomData,
        }
    }

    /// Returns the dimensions used for each face in the texture cube.
    pub const fn dim(&self) -> [u32; 2] {
        self.dim
    }
}

/// Contains information about the a face of a texture cube.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureCubeFace {
    /// Index of the face in the texture cube (0-5).
    pub face_index: u8,
    /// Forward direction vector for the face (relative to the center of the cube).
    pub forward_vector: [i8; 3],
    /// Up direction vector for the face (relative to the center of the cube).
    pub up_vector: [i8; 3],
}
impl TextureCubeFace {
    /// Returns the data per face of the texture cube.
    pub const FACES: [TextureCubeFace; 6] = [
        TextureCubeFace {
            face_index: 0,
            forward_vector: [1, 0, 0],
            up_vector: [0, 1, 0],
        },
        TextureCubeFace {
            face_index: 1,
            forward_vector: [-1, 0, 0],
            up_vector: [0, 1, 0],
        },
        TextureCubeFace {
            face_index: 2,
            forward_vector: [0, 1, 0],
            up_vector: [0, 0, -1],
        },
        TextureCubeFace {
            face_index: 3,
            forward_vector: [0, -1, 0],
            up_vector: [0, 0, 1],
        },
        TextureCubeFace {
            face_index: 4,
            forward_vector: [0, 0, 1],
            up_vector: [0, 1, 0],
        },
        TextureCubeFace {
            face_index: 5,
            forward_vector: [0, 0, -1],
            up_vector: [0, 1, 0],
        },
    ];
}

/// Represents a cube texture with six faces, each face being a 2D texture.
///
/// For details about each face, see [`TextureCubeFace::FACES`].
pub struct TextureCube<V> {
    textures: [Texture<V, 2>; 6],
}
impl<V> TextureCube<V> {
    pub fn new(face_dim: [u32; 2]) -> Self
    where
        V: Default,
    {
        let textures = [
            Texture::new(face_dim),
            Texture::new(face_dim),
            Texture::new(face_dim),
            Texture::new(face_dim),
            Texture::new(face_dim),
            Texture::new(face_dim),
        ];
        Self { textures }
    }

    pub fn from_images(textures: [Texture<V, 2>; 6]) -> Self {
        // Ensure all textures have the same dimensions
        let first_dim = textures[0].dim();
        for texture in &textures[1..] {
            assert_eq!(
                texture.dim(),
                first_dim,
                "All textures must have the same dimensions"
            );
        }
        Self { textures }
    }

    pub fn face_dim(&self) -> [u32; 2] {
        self.textures[0].dim()
    }

    pub fn textures(&self) -> &[Texture<V, 2>] {
        &self.textures
    }

    pub fn as_textures_mut<'a>(&'a mut self) -> Vec<TextureMut<'a, V, 2>> {
        self.textures
            .iter_mut()
            .map(|t| t.as_texture_mut())
            .collect()
    }

    pub fn as_texture_ref_array<'a>(&'a self) -> TextureRefArray<'a, V, 2> {
        TextureRefArray {
            textures: self.textures.iter().map(|t| t.as_texture_ref()).collect(),
        }
    }
    pub fn as_texture_cube_ref<'a>(&'a self) -> TextureCubeRef<'a, V> {
        TextureCubeRef {
            textures: [
                self.textures[0].as_texture_ref(),
                self.textures[1].as_texture_ref(),
                self.textures[2].as_texture_ref(),
                self.textures[3].as_texture_ref(),
                self.textures[4].as_texture_ref(),
                self.textures[5].as_texture_ref(),
            ],
        }
    }
}
impl<P> IndexAsType for TextureCube<P> {
    type Value = Texture<P, 2>;
}

/// Represents a cube texture with six faces, each face being a 2D texture.
///
/// For details about each face, see [`TextureCubeFace::FACES`].
pub struct TextureCubeRef<'a, V> {
    textures: [TextureRef<'a, V, 2>; 6],
}
impl<'a, V> TextureCubeRef<'a, V> {
    pub fn new(textures: [TextureRef<'a, V, 2>; 6]) -> Self {
        // Ensure all textures have the same dimensions
        let first_dim = textures[0].dim();
        for texture in &textures[1..] {
            assert_eq!(
                texture.dim(),
                first_dim,
                "All textures must have the same dimensions"
            );
        }
        Self { textures }
    }

    pub fn face_dim(&self) -> [u32; 2] {
        self.textures[0].dim()
    }

    pub fn textures(&self) -> &[TextureRef<'a, V, 2>] {
        &self.textures
    }

    pub fn as_texture_ref_array(&self) -> TextureRefArray<'a, V, 2> {
        TextureRefArray {
            textures: self.textures.iter().map(|t| t.clone()).collect(),
        }
    }
}
impl<'a, V> Clone for TextureCubeRef<'a, V> {
    fn clone(&self) -> Self {
        Self {
            textures: self.textures.clone(),
        }
    }
}

/*
================================================================================
DeviceLoad implementations for textures
================================================================================
*/
macro_rules! impl_texture_load {
    ($dim:expr, $name:ident, $allocName:ident) => {
        impl<B: Backend, P: PixelType<B> + 'static> DeviceLoad<B> for TextureAlloc<P, $dim> {
            type LoadAs = Texture<P, $dim>;

            fn load(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let data: TextureData<'static, P> = TextureData::$allocName(data[0].dim);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }

            fn reload(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                dest: &Stored<B, Self::LoadAs>,
            ) -> Result<(), <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let data: TextureData<'static, P> = TextureData::$allocName(data[0].dim);
                d.reload_texture_data(data, dest.stored_data())
            }

            fn load_sendable(
                d: &mut <B as Backend>::SendDeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let data: TextureData<'static, P> = TextureData::$allocName(data[0].dim);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }
        }

        impl<B: Backend, P: PixelType<B>> DeviceLoad<B> for Texture<P, $dim> {
            type LoadAs = Texture<P, $dim>;

            fn load(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture = data[0].as_texture_ref();
                let data = TextureData::$name(texture);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }

            fn reload(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                dest: &Stored<B, Self::LoadAs>,
            ) -> Result<(), <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture = data[0].as_texture_ref();
                let data = TextureData::$name(texture);
                d.reload_texture_data(data, dest.stored_data())
            }

            fn load_sendable(
                d: &mut <B as Backend>::SendDeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture = data[0].as_texture_ref();
                let data = TextureData::$name(texture);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }
        }
        impl<'a, B: Backend, P: PixelType<B>> DeviceLoad<B> for TextureRef<'a, P, $dim> {
            type LoadAs = Texture<P, $dim>;

            fn load(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture = data[0].as_texture_ref();
                let data = TextureData::$name(texture);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }

            fn reload(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                dest: &Stored<B, Self::LoadAs>,
            ) -> Result<(), <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture = data[0].as_texture_ref();
                let data = TextureData::$name(texture);
                d.reload_texture_data(data, dest.stored_data())
            }

            fn load_sendable(
                d: &mut <B as Backend>::SendDeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture = data[0].as_texture_ref();
                let data = TextureData::$name(texture);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }
        }

        impl<B: Backend, P: PixelType<B>> DeviceDownload<B> for Texture<P, $dim> {
            type DownloadAs = Self;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                unsafe { d.download_texture::<P, $dim>(data.stored_data()) }
            }
        }
    };
}
impl_texture_load!(1, Texture1d, AllocTexture1d);
impl_texture_load!(2, Texture2d, AllocTexture2d);
impl_texture_load!(3, Texture3d, AllocTexture3d);

macro_rules! impl_texture_array_load {
    ($dim:expr, $name:ident, $alloc_name:ident) => {
        impl<B: Backend, P: PixelType<B> + 'static> DeviceLoad<B> for TextureArrayAlloc<P, $dim> {
            type LoadAs = TextureArray<P, $dim>;

            fn load(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let data: TextureData<'static, P> =
                    TextureData::$alloc_name(data[0].dim, data[0].len);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }

            fn reload(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                dest: &Stored<B, Self::LoadAs>,
            ) -> Result<(), <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let data: TextureData<'static, P> =
                    TextureData::$alloc_name(data[0].dim, data[0].len);
                d.reload_texture_data(data, dest.stored_data())
            }

            fn load_sendable(
                d: &mut <B as Backend>::SendDeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let data: TextureData<'static, P> =
                    TextureData::$alloc_name(data[0].dim, data[0].len);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }
        }

        impl<P> IndexAsType for TextureArray<P, $dim> {
            type Value = Texture<P, $dim>;
        }

        impl<B: Backend, P: PixelType<B>> DeviceDownload<B> for TextureArray<P, $dim> {
            type DownloadAs = Self;

            unsafe fn download(
                d: &mut B::DeviceInstance,
                data: &Stored<B, Self>,
            ) -> Result<Self::DownloadAs, B::Error> {
                unsafe { d.download_texture_array::<P, $dim>(data.stored_data()) }
            }
        }

        impl<B: Backend, P: PixelType<B>> DeviceLoad<B> for TextureArray<P, $dim> {
            type LoadAs = TextureArray<P, $dim>;

            fn load(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture_array = data[0].as_texture_ref_array();
                let data = TextureData::$name(texture_array);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }

            fn reload(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                dest: &Stored<B, Self::LoadAs>,
            ) -> Result<(), <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture_array = data[0].as_texture_ref_array();
                let data = TextureData::$name(texture_array);
                d.reload_texture_data(data, dest.stored_data())
            }

            fn load_sendable(
                d: &mut <B as Backend>::SendDeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture_array = data[0].as_texture_ref_array();
                let data = TextureData::$name(texture_array);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }
        }

        impl<'a, B: Backend, P: PixelType<B>> DeviceLoad<B> for TextureRefArray<'a, P, $dim> {
            type LoadAs = TextureArray<P, $dim>;

            fn load(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture_array = data[0].as_texture_ref_array();
                let data = TextureData::$name(texture_array);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }

            fn reload(
                d: &mut <B as Backend>::DeviceInstance,
                data: &[Self],
                dest: &Stored<B, Self::LoadAs>,
            ) -> Result<(), <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture_array = data[0].as_texture_ref_array();
                let data = TextureData::$name(texture_array);
                d.reload_texture_data(data, dest.stored_data())
            }

            fn load_sendable(
                d: &mut <B as Backend>::SendDeviceInstance,
                data: &[Self],
                options: <B as Backend>::LoadOptions,
            ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
                if data.len() != 1 {
                    return Err(B::error_single_value_only());
                }
                let texture_array = data[0].as_texture_ref_array();
                let data = TextureData::$name(texture_array);
                let stored = d.load_texture_data(data, options)?;
                unsafe { Ok(stored.unchecked_cast()) }
            }
        }
    };
}
impl_texture_array_load!(1, Texture1dArray, AllocTexture1dArray);
impl_texture_array_load!(2, Texture2dArray, AllocTexture2dArray);

impl<B: Backend, P: PixelType<B> + 'static> DeviceLoad<B> for TextureCubeAlloc<P> {
    type LoadAs = TextureCube<P>;

    fn load(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            return Err(B::error_single_value_only());
        }
        let data: TextureData<'static, P> = TextureData::AllocTextureCube(data[0].dim);
        let stored = d.load_texture_data(data, options)?;
        unsafe { Ok(stored.unchecked_cast()) }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        if data.len() != 1 {
            return Err(B::error_single_value_only());
        }
        let data: TextureData<'static, P> = TextureData::AllocTextureCube(data[0].dim);
        d.reload_texture_data(data, dest.stored_data())
    }

    fn load_sendable(
        d: &mut <B as Backend>::SendDeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            return Err(B::error_single_value_only());
        }
        let data: TextureData<'static, P> = TextureData::AllocTextureCube(data[0].dim);
        let stored = d.load_texture_data(data, options)?;
        unsafe { Ok(stored.unchecked_cast()) }
    }
}

impl<B: Backend, P: PixelType<B>> DeviceLoad<B> for TextureCube<P> {
    type LoadAs = TextureCube<P>;

    fn load(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            return Err(B::error_single_value_only());
        }
        let texture_cube = data[0].as_texture_cube_ref();
        let data = TextureData::TextureCube(texture_cube);
        let stored = d.load_texture_data(data, options)?;
        unsafe { Ok(stored.unchecked_cast()) }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        if data.len() != 1 {
            return Err(B::error_single_value_only());
        }
        let texture_cube = data[0].as_texture_cube_ref();
        let data = TextureData::TextureCube(texture_cube);
        d.reload_texture_data(data, dest.stored_data())
    }

    fn load_sendable(
        d: &mut <B as Backend>::SendDeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            return Err(B::error_single_value_only());
        }
        let texture_cube = data[0].as_texture_cube_ref();
        let data = TextureData::TextureCube(texture_cube);
        let stored = d.load_texture_data(data, options)?;
        unsafe { Ok(stored.unchecked_cast()) }
    }
}

impl<'a, B: Backend, P: PixelType<B>> DeviceLoad<B> for TextureCubeRef<'a, P> {
    type LoadAs = TextureCube<P>;

    fn load(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            return Err(B::error_single_value_only());
        }
        let texture_cube = data[0].clone();
        let data = TextureData::TextureCube(texture_cube);
        let stored = d.load_texture_data(data, options)?;
        unsafe { Ok(stored.unchecked_cast()) }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        if data.len() != 1 {
            return Err(B::error_single_value_only());
        }
        let texture_cube = data[0].clone();
        let data = TextureData::TextureCube(texture_cube);
        d.reload_texture_data(data, dest.stored_data())
    }

    fn load_sendable(
        d: &mut <B as Backend>::SendDeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            return Err(B::error_single_value_only());
        }
        let texture_cube = data[0].clone();
        let data = TextureData::TextureCube(texture_cube);
        let stored = d.load_texture_data(data, options)?;
        unsafe { Ok(stored.unchecked_cast()) }
    }
}

impl<B: Backend, P: PixelType<B>> DeviceDownload<B> for TextureCube<P> {
    type DownloadAs = Self;

    unsafe fn download(
        d: &mut B::DeviceInstance,
        data: &Stored<B, Self>,
    ) -> Result<Self::DownloadAs, B::Error> {
        unsafe { d.download_texture_cube::<P>(data.stored_data()) }
    }
}
