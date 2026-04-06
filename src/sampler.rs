use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum SamplerType {
    Nearest,
    Linear,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum SamplerRepeat {
    Clamp,
    Repeat,
    Mirror,
}

/// Defines a basic sampler for textures. This allows for some of the more common sampler types and is not designed to be a
/// comprehensive list of all possible sampler types. It is expected that most backends will create additional samplers that
/// enable the specific capabilities of that backend.
#[derive(Clone, Debug)]
pub struct Sampler {
    minify_type: SamplerType,
    magnify_type: SamplerType,
    repeat: SamplerRepeat,
}
impl Sampler {
    pub fn new(minify_type: SamplerType, magnify_type: SamplerType, repeat: SamplerRepeat) -> Self {
        Self {
            minify_type,
            magnify_type,
            repeat,
        }
    }
    pub fn nearest_clamp() -> Self {
        Self::new(
            SamplerType::Nearest,
            SamplerType::Nearest,
            SamplerRepeat::Clamp,
        )
    }
    pub fn nearest_repeat() -> Self {
        Self::new(
            SamplerType::Nearest,
            SamplerType::Nearest,
            SamplerRepeat::Repeat,
        )
    }
    pub fn nearest_mirror() -> Self {
        Self::new(
            SamplerType::Nearest,
            SamplerType::Nearest,
            SamplerRepeat::Mirror,
        )
    }
    pub fn linear_clamp() -> Self {
        Self::new(
            SamplerType::Linear,
            SamplerType::Linear,
            SamplerRepeat::Clamp,
        )
    }
    pub fn linear_repeat() -> Self {
        Self::new(
            SamplerType::Linear,
            SamplerType::Linear,
            SamplerRepeat::Repeat,
        )
    }
    pub fn linear_mirror() -> Self {
        Self::new(
            SamplerType::Linear,
            SamplerType::Linear,
            SamplerRepeat::Mirror,
        )
    }

    pub fn minify_type(&self) -> SamplerType {
        self.minify_type
    }
    pub fn magnify_type(&self) -> SamplerType {
        self.magnify_type
    }
    pub fn repeat(&self) -> SamplerRepeat {
        self.repeat
    }
}
impl<B: Backend> DeviceLoad<B> for Sampler {
    type LoadAs = Self;

    fn load(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
        let result = d.load_sampler(data[0].clone(), options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        d.reload_sampler(data[0].clone(), dest.stored_data())
    }

    fn load_sendable(
        d: &mut <B as Backend>::SendDeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
        let result = d.load_sampler(data[0].clone(), options)?;
        unsafe { Ok(result.unchecked_cast()) }
    }
}
