use crate::*;

pub struct Surface<B: Backend> {
    device: Device<B>,
    surface: B::SurfaceInstance,
    fps_counter: Option<FPSCounter>,
    fps: u32,
}
impl<B: Backend> Surface<B> {
    pub(crate) fn new(init: B::InitializationData) -> Result<Self, B::Error> {
        let sync_memory = SyncMemoryStorage::new();
        let memory_storage = MemoryStorage::new(sync_memory);
        let index_manager = memory_storage.get_index_manager();
        let (device_instance, surface_instance) =
            B::create_device_and_surface(init, memory_storage)?;
        let device: Device<B> = Device::new(device_instance, index_manager.clone());
        index_manager.set_device(device.clone());
        let mut surface = Surface {
            device,
            surface: surface_instance,
            fps_counter: None,
            fps: 0,
        };
        surface.internal(|d, s| B::initialize_device_and_surface(d, s))?;
        Ok(surface)
    }

    pub fn device(&self) -> &Device<B> {
        &self.device
    }

    pub fn internal<T>(
        &mut self,
        action: impl FnOnce(&mut B::DeviceInstance, &mut B::SurfaceInstance) -> T,
    ) -> T {
        let surface = &mut self.surface;
        self.device.internal(|d| action(d, surface))
    }

    /// Enables or disables FPS tracking. When enabled, the surface will track the frames per second (FPS) and make it available
    /// through the `fps()` method.
    pub fn track_fps(&mut self, enabled: bool) {
        if enabled {
            if self.fps_counter.is_none() {
                self.fps_counter = Some(FPSCounter::new());
                self.fps = 0;
            }
        } else {
            self.fps_counter = None;
            self.fps = 0;
        }
    }

    /// Returns the current frames per second (FPS). Must be enabled with `track_fps(true)`.
    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn begin_frame<'surface>(
        &'surface mut self,
    ) -> Result<SurfaceFrame<'surface, B>, B::Error> {
        if let Some(counter) = self.fps_counter.as_mut() {
            self.fps = counter.tick() as u32;
        }
        let surface = &mut self.surface;
        let frame = self.device.internal(|d| B::begin_frame(d, surface))?;
        Ok(SurfaceFrame {
            device: &self.device,
            surface,
            frame: Some(frame),
            fps: self.fps,
        })
    }
}

pub struct SurfaceFrame<'surface, B: Backend> {
    device: &'surface Device<B>,
    surface: &'surface mut B::SurfaceInstance,
    frame: Option<B::FrameInstance>,
    fps: u32,
}
impl<'surface, B: Backend> SurfaceFrame<'surface, B> {
    /// Returns the primary, active render target for this surface.
    pub fn render_target(&mut self) -> Result<Stored<B, Texture<Srgba, 2>>, B::Error> {
        let frame = self.frame.as_mut().unwrap();
        self.device
            .internal(|d| frame.render_target(d, self.surface))
    }

    /// Returns the active depth-only render target for this surface. This is simply a convenience method for
    /// `frame.named_target("default-depth")`.
    pub fn depth_target(&mut self) -> Result<Stored<B, Texture<Depth, 2>>, B::Error> {
        self.named_target("default-depth")
    }

    /// Returns the active depth-only render target for this surface. This is simply a convenience method for
    /// `frame.named_target_using("default-depth", usage)`.
    pub fn depth_target_using<Opts: BackendLoadOptions<B>>(
        &mut self,
        usage: Opts,
    ) -> Result<Stored<B, Texture<Depth, 2>>, B::Error> {
        self.named_target_using("default-depth", usage)
    }

    /// Returns the active texture render target for this surface and name. If the target does not exist, it will be created.
    /// The texture returned will have the same dimensions as the surface.
    ///
    /// If the pixel type does not match the pixel type of the named target, an error will be returned.
    pub fn named_target<P: PixelType<B>>(
        &mut self,
        name: &str,
    ) -> Result<Stored<B, Texture<P, 2>>, B::Error> {
        let frame = self.frame.as_mut().unwrap();
        self.device
            .internal(|d| frame.named_target::<P>(d, self.surface, name, B::default_load_options()))
    }

    /// Returns the active texture render target for this surface and name. If the target does not exist, it will be created.
    /// The texture returned will have the same dimensions as the surface.
    ///
    /// If the pixel type does not match the pixel type of the named target, an error will be returned.
    pub fn named_target_using<P: PixelType<B>, Opts: BackendLoadOptions<B>>(
        &mut self,
        name: &str,
        options: Opts,
    ) -> Result<Stored<B, Texture<P, 2>>, B::Error> {
        let frame = self.frame.as_mut().unwrap();
        self.device.internal(|d| {
            frame.named_target::<P>(d, self.surface, name, options.create_load_options()?)
        })
    }

    /// Discards all named targets. This should be called when the named targets are no longer needed, to free up resources.
    ///
    /// For example, a game might call this when switching from the main menu to gameplay.
    pub fn discard_named_targets(&mut self) -> Result<(), B::Error> {
        let frame = self.frame.as_mut().unwrap();
        self.device
            .internal(|d| frame.discard_named_targets(d, self.surface))
    }

    pub fn begin_compute<'frame>(&'frame mut self) -> Result<ComputePass<'frame, B>, B::Error> {
        let frame = self.frame.as_mut().unwrap();
        let pass = self
            .device
            .internal(|d| B::begin_compute_pass(d, self.surface, frame, NestedPass::Frame))?;
        Ok(ComputePass {
            device: self.device,
            surface: self.surface,
            frame,
            pass: Some(pass),
        })
    }

    pub fn begin_render<'frame>(
        &'frame mut self,
        target: impl Into<StoredData<B>>,
    ) -> RenderPassBuilder<'frame, B> {
        let frame = self.frame.as_mut().unwrap();
        RenderPassBuilder::new(
            self.device,
            self.surface,
            frame,
            NestedPass::Frame,
            target.into(),
            RenderTargetClear::Default,
        )
    }
    pub fn begin_render_with_clear<'frame>(
        &'frame mut self,
        target: impl Into<StoredData<B>>,
        clear: impl Into<RenderTargetClear>,
    ) -> RenderPassBuilder<'frame, B> {
        let frame = self.frame.as_mut().unwrap();
        RenderPassBuilder::new(
            self.device,
            self.surface,
            frame,
            NestedPass::Frame,
            target.into(),
            clear.into(),
        )
    }

    /// Returns the current frames per second (FPS). Must be enablded with `surface.track_fps(true)`.
    pub fn fps(&self) -> u32 {
        self.fps
    }

    pub fn finish(mut self) -> Result<(), B::Error> {
        self.internal_finish(false)
    }

    fn internal_finish(&mut self, dropped_without_finish: bool) -> Result<(), B::Error> {
        if let Some(frame) = self.frame.take() {
            self.device
                .internal(|d| B::end_frame(d, self.surface, frame, dropped_without_finish))?;
        }
        Ok(())
    }
}
impl<'surface, B: Backend> std::ops::Drop for SurfaceFrame<'surface, B> {
    fn drop(&mut self) {
        if self.frame.is_some() {
            // If the frame was not finished explicitly, we finalize it here.
            let _ignore = self.internal_finish(true);
        }
    }
}

pub struct ComputePass<'a, B: Backend> {
    pub(crate) device: &'a Device<B>,
    pub(crate) surface: &'a mut B::SurfaceInstance,
    pub(crate) frame: &'a mut B::FrameInstance,
    pub(crate) pass: Option<B::ComputePassInstance>,
}
impl<'a, B: Backend> ComputePass<'a, B> {
    pub fn run<'s>(&'s mut self, pipeline: &'s Pipeline<B>) -> ActivePipeline<'a, 's, B> {
        ActivePipeline::new_compute(self.device, self, pipeline)
    }

    pub fn begin_compute<'nested>(&'nested mut self) -> Result<ComputePass<'nested, B>, B::Error> {
        let pass = self.pass.as_mut().unwrap();
        let pass = self.device.internal(|d| {
            B::begin_compute_pass(d, self.surface, self.frame, NestedPass::Compute(pass))
        })?;
        Ok(ComputePass {
            device: self.device,
            surface: self.surface,
            frame: self.frame,
            pass: Some(pass),
        })
    }

    pub fn begin_render<'nested>(
        &'nested mut self,
        target: impl Into<StoredData<B>>,
    ) -> RenderPassBuilder<'nested, B> {
        let pass = self.pass.as_mut().unwrap();
        RenderPassBuilder::new(
            self.device,
            self.surface,
            self.frame,
            NestedPass::Compute(pass),
            target.into(),
            RenderTargetClear::Default,
        )
    }
    pub fn begin_render_with_clear<'nested>(
        &'nested mut self,
        target: impl Into<StoredData<B>>,
        clear: impl Into<RenderTargetClear>,
    ) -> RenderPassBuilder<'nested, B> {
        let pass = self.pass.as_mut().unwrap();
        RenderPassBuilder::new(
            self.device,
            self.surface,
            self.frame,
            NestedPass::Compute(pass),
            target.into(),
            clear.into(),
        )
    }

    pub fn finish(mut self) -> Result<(), B::Error> {
        self.internal_finish(false)
    }

    fn internal_finish(&mut self, dropped_without_finish: bool) -> Result<(), B::Error> {
        // This method can be used to finalize the compute pass with specific backend logic.
        if let Some(pass) = self.pass.take() {
            self.device.internal(|d| {
                B::end_compute_pass(d, self.surface, self.frame, pass, dropped_without_finish)
            })?;
        }
        Ok(())
    }
}
impl<'a, B: Backend> std::ops::Drop for ComputePass<'a, B> {
    fn drop(&mut self) {
        if self.pass.is_some() {
            // If the compute pass was not finished explicitly, we finalize it here.
            let _ignore = self.internal_finish(true);
        }
    }
}

pub struct RenderPassBuilder<'a, B: Backend> {
    device: &'a Device<B>,
    surface: &'a mut B::SurfaceInstance,
    frame: &'a mut B::FrameInstance,
    nested: NestedPass<'a, B>,
    targets: Vec<(StoredData<B>, RenderTargetClear)>,
}
impl<'a, B: Backend> RenderPassBuilder<'a, B> {
    fn new(
        device: &'a Device<B>,
        surface: &'a mut B::SurfaceInstance,
        frame: &'a mut B::FrameInstance,
        nested: NestedPass<'a, B>,
        target: StoredData<B>,
        clear: RenderTargetClear,
    ) -> Self {
        let mut targets = Vec::with_capacity(8);
        targets.push((target, clear));
        RenderPassBuilder {
            device,
            surface,
            frame,
            nested,
            targets,
        }
    }

    pub fn add(mut self, target: impl Into<StoredData<B>>) -> Self {
        self.targets
            .push((target.into(), RenderTargetClear::Default));
        self
    }
    pub fn add_with_clear(
        mut self,
        target: impl Into<StoredData<B>>,
        clear: impl Into<RenderTargetClear>,
    ) -> Self {
        self.targets.push((target.into(), clear.into()));
        self
    }

    pub fn start(self) -> Result<RenderPass<'a, B>, B::Error> {
        let pass = self.device.internal(|d| {
            B::begin_render_pass(d, self.surface, self.frame, self.nested, self.targets)
        })?;
        Ok(RenderPass {
            device: self.device,
            surface: self.surface,
            frame: self.frame,
            pass: Some(pass),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderTargetClear {
    Default,
    None,
    Color(LrgbaF32),
    Depth(f32),
    Stencil(u32),
    DepthStencil(f32, u32),
}
impl From<Srgba> for RenderTargetClear {
    fn from(color: Srgba) -> Self {
        RenderTargetClear::Color(color.to_lrgba())
    }
}
impl From<LrgbaF32> for RenderTargetClear {
    fn from(color: LrgbaF32) -> Self {
        RenderTargetClear::Color(color)
    }
}
impl From<[f32; 4]> for RenderTargetClear {
    fn from(color: [f32; 4]) -> Self {
        RenderTargetClear::Color(color.into())
    }
}
impl From<f32> for RenderTargetClear {
    fn from(depth: f32) -> Self {
        RenderTargetClear::Depth(depth)
    }
}
impl From<u32> for RenderTargetClear {
    fn from(stencil: u32) -> Self {
        RenderTargetClear::Stencil(stencil)
    }
}

pub struct RenderPass<'a, B: Backend> {
    device: &'a Device<B>,
    pub(crate) surface: &'a mut B::SurfaceInstance,
    pub(crate) frame: &'a mut B::FrameInstance,
    pub(crate) pass: Option<B::RenderPassInstance>,
}
impl<'a, B: Backend> RenderPass<'a, B> {
    pub fn run<'s>(&'s mut self, pipeline: &'s Pipeline<B>) -> ActivePipeline<'a, 's, B> {
        ActivePipeline::new_render(self.device, self, pipeline)
    }

    pub fn begin_compute<'nested>(&'nested mut self) -> Result<ComputePass<'nested, B>, B::Error> {
        let pass = self.pass.as_mut().unwrap();
        let pass = self.device.internal(|d| {
            B::begin_compute_pass(d, self.surface, self.frame, NestedPass::Render(pass))
        })?;
        Ok(ComputePass {
            device: self.device,
            surface: self.surface,
            frame: self.frame,
            pass: Some(pass),
        })
    }

    pub fn begin_render<'nested>(
        &'nested mut self,
        target: impl Into<StoredData<B>>,
    ) -> RenderPassBuilder<'nested, B> {
        let pass = self.pass.as_mut().unwrap();
        RenderPassBuilder::new(
            self.device,
            self.surface,
            self.frame,
            NestedPass::Render(pass),
            target.into(),
            RenderTargetClear::Default,
        )
    }
    pub fn begin_render_with_clear<'nested>(
        &'nested mut self,
        target: impl Into<StoredData<B>>,
        clear: impl Into<RenderTargetClear>,
    ) -> RenderPassBuilder<'nested, B> {
        let pass = self.pass.as_mut().unwrap();
        RenderPassBuilder::new(
            self.device,
            self.surface,
            self.frame,
            NestedPass::Render(pass),
            target.into(),
            clear.into(),
        )
    }

    pub fn finish(mut self) -> Result<(), B::Error> {
        self.internal_finish(false)
    }

    fn internal_finish(&mut self, dropped_without_finish: bool) -> Result<(), B::Error> {
        // This method can be used to finalize the render pass with specific backend logic.
        if let Some(pass) = self.pass.take() {
            self.device.internal(|d| {
                B::end_render_pass(d, self.surface, self.frame, pass, dropped_without_finish)
            })?;
        }
        Ok(())
    }
}
impl<'a, B: Backend> std::ops::Drop for RenderPass<'a, B> {
    fn drop(&mut self) {
        if self.pass.is_some() {
            // If the render pass was not finished explicitly, we finalize it here.
            let _ignore = self.internal_finish(true);
        }
    }
}

#[cfg(feature = "web-time")]
type Instant = web_time::Instant;
#[cfg(not(feature = "web-time"))]
type Instant = std::time::Instant;

#[cfg(feature = "web-time")]
type Duration = web_time::Duration;
#[cfg(not(feature = "web-time"))]
type Duration = std::time::Duration;

/// Copied from the `fps_counter` crate: Measures Frames Per Second (FPS).
#[derive(Debug)]
struct FPSCounter {
    /// The last registered frames.
    last_second_frames: std::collections::VecDeque<Instant>,
}
impl FPSCounter {
    /// Creates a new FPSCounter.
    pub fn new() -> FPSCounter {
        FPSCounter {
            last_second_frames: std::collections::VecDeque::with_capacity(128),
        }
    }

    /// Updates the FPSCounter and returns number of frames.
    pub fn tick(&mut self) -> usize {
        // Note: Subtraction operations can be problem on web targets (underflow)
        let now = Instant::now();

        while self
            .last_second_frames
            .front()
            .map_or(false, |t| *t + Duration::from_secs(1) < now)
        {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames.push_back(now);
        self.last_second_frames.len()
    }
}

pub enum NestedPass<'nested, B: Backend> {
    Frame,
    Compute(&'nested mut B::ComputePassInstance),
    Render(&'nested mut B::RenderPassInstance),
}
