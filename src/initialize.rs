use crate::{Backend, Surface};

/// Initializes a surface using the provided backend's initialization data.
/// 
/// Once the surface is initialized, the device can be pulled from that (`surface.device()`).
pub fn init_surface<B: Backend>(init: B::InitializationData) -> Result<Surface<B>, B::Error> {
    crate::Surface::new(init)
}
