use crate::*;

/// Describes a backend graphics system.
pub trait Backend: Sized + 'static {
    type InitializationData;
    type DeviceInstance: BackendDeviceInstance<Self>;
    type SendDeviceInstance: BackendSendDeviceInstance<Self>;

    type SurfaceInstance;
    type FrameInstance: BackendFrameInstance<Self>;
    type ComputePassInstance;
    type RenderPassInstance;

    type PipelineBuilder<'ctxt>: BackendPipelineBuilder<'ctxt, Self>;

    type StoredValue: std::fmt::Debug;
    type SendStoredValue: Send + std::fmt::Debug + 'static;

    type CompiledPipeline;

    type LoadOptions: BackendLoadOptions<Self> + Clone;
    type PixelLayout;
    type Error: std::error::Error;

    /// The value to hold in place of a deleted entry within the memory manager.
    ///
    /// Note: This requirement may be removed in the future, but for now it simplifies the memory management system.
    fn deleted_value() -> Self::StoredValue;

    fn error_unsupported(feature: &'static str) -> Self::Error;
    fn error_single_value_only() -> Self::Error;

    fn default_load_options() -> Self::LoadOptions;
    /// Returns the default load options for pipeline parameters (ex. `load_input` and `load_array_input`).
    fn default_parameter_load_options() -> Self::LoadOptions {
        Self::default_load_options()
    }

    fn create_device_and_surface(
        init: Self::InitializationData,
        storage: MemoryStorage<Self>,
    ) -> Result<(Self::DeviceInstance, Self::SurfaceInstance), Self::Error>;

    fn initialize_device_and_surface(
        device: &mut Self::DeviceInstance,
        surface: &mut Self::SurfaceInstance,
    ) -> Result<(), Self::Error> {
        let _ = (device, surface);
        Ok(())
    }

    fn create_background_loader(
        d: &mut Self::DeviceInstance,
    ) -> Result<Self::SendDeviceInstance, Self::Error>;

    fn begin_frame<'surface>(
        d: &mut Self::DeviceInstance,
        surface: &'surface mut Self::SurfaceInstance,
    ) -> Result<Self::FrameInstance, Self::Error>;

    fn end_frame<'surface>(
        d: &mut Self::DeviceInstance,
        surface: &'surface mut Self::SurfaceInstance,
        frame: Self::FrameInstance,
        dropped_without_finish: bool,
    ) -> Result<(), Self::Error>;

    fn begin_render_pass<'surface, 'frame, 'nested>(
        d: &mut Self::DeviceInstance,
        surface: &'surface mut Self::SurfaceInstance,
        frame: &'frame mut Self::FrameInstance,
        nested: NestedPass<'nested, Self>,
        targets: Vec<(StoredData<Self>, RenderTargetClear)>,
    ) -> Result<Self::RenderPassInstance, Self::Error>;

    fn end_render_pass<'surface, 'frame>(
        d: &mut Self::DeviceInstance,
        surface: &'surface mut Self::SurfaceInstance,
        frame: &'frame mut Self::FrameInstance,
        render_pass: Self::RenderPassInstance,
        dropped_without_finish: bool,
    ) -> Result<(), Self::Error>;

    fn run_render_pipeline<'surface, 'frame>(
        d: &mut Self::DeviceInstance,
        surface: &'surface mut Self::SurfaceInstance,
        frame: &'frame mut Self::FrameInstance,
        render_pass: &mut Self::RenderPassInstance,
        pipeline: &mut Self::CompiledPipeline,
        dyn_params: &[(StoredData<Self>, ParameterAccessMode)],
    ) -> Result<(), Self::Error>;

    fn begin_compute_pass<'surface, 'frame, 'nested>(
        d: &mut Self::DeviceInstance,
        surface: &'surface mut Self::SurfaceInstance,
        frame: &'frame mut Self::FrameInstance,
        nested: NestedPass<'nested, Self>,
    ) -> Result<Self::ComputePassInstance, Self::Error>;

    fn end_compute_pass<'surface, 'frame>(
        d: &mut Self::DeviceInstance,
        surface: &'surface mut Self::SurfaceInstance,
        frame: &'frame mut Self::FrameInstance,
        compute_pass: Self::ComputePassInstance,
        dropped_without_finish: bool,
    ) -> Result<(), Self::Error>;

    fn run_compute_pipeline<'surface, 'frame>(
        d: &mut Self::DeviceInstance,
        surface: &'surface mut Self::SurfaceInstance,
        frame: &'frame mut Self::FrameInstance,
        compute_pass: &mut Self::ComputePassInstance,
        pipeline: &mut Self::CompiledPipeline,
        dyn_params: &[(StoredData<Self>, ParameterAccessMode)],
    ) -> Result<(), Self::Error>;

    fn pixel_layout(p: PixelFormat) -> Result<Self::PixelLayout, Self::Error>;
    fn usage_flags_as_options(flags: UsageFlags) -> Result<Self::LoadOptions, Self::Error>;
}

pub trait BackendDeviceInstance<B: Backend> {
    fn load_buffer_data(
        &mut self,
        data: BufferData<'_>,
        options: B::LoadOptions,
    ) -> Result<StoredData<B>, B::Error>;
    fn reload_buffer_data(
        &mut self,
        data: BufferData<'_>,
        dest: &StoredData<B>,
    ) -> Result<(), B::Error>;

    fn load_texture_data<P: PixelType<B>>(
        &mut self,
        data: TextureData<'_, P>,
        options: B::LoadOptions,
    ) -> Result<StoredData<B>, B::Error>;
    fn reload_texture_data<P: PixelType<B>>(
        &mut self,
        data: TextureData<'_, P>,
        dest: &StoredData<B>,
    ) -> Result<(), B::Error>;

    fn load_sampler(
        &mut self,
        sampler: Sampler,
        options: B::LoadOptions,
    ) -> Result<StoredData<B>, B::Error>;
    fn reload_sampler(&mut self, sampler: Sampler, dest: &StoredData<B>) -> Result<(), B::Error>;

    fn load_program(
        &mut self,
        program: &ProgramRef,
        options: B::LoadOptions,
    ) -> Result<StoredData<B>, B::Error>;
    fn reload_program(
        &mut self,
        program: &ProgramRef,
        dest: &StoredData<B>,
    ) -> Result<(), B::Error>;

    fn load_render_ranges(
        &mut self,
        ranges: RenderRanges,
        options: B::LoadOptions,
    ) -> Result<StoredData<B>, B::Error>;
    fn reload_render_ranges(
        &mut self,
        ranges: RenderRanges,
        dest: &StoredData<B>,
    ) -> Result<(), B::Error>;

    fn load_compute_workgroup(
        &mut self,
        workgroup: ComputeWorkgroup,
        options: B::LoadOptions,
    ) -> Result<StoredData<B>, B::Error>;
    fn reload_compute_workgroup(
        &mut self,
        workgroup: ComputeWorkgroup,
        dest: &StoredData<B>,
    ) -> Result<(), B::Error>;

    /// Downloads a buffer to the CPU.
    ///
    /// Performance: This method should prioritize successful downloads over high performance.
    unsafe fn download_buffer<V>(&mut self, data: &StoredData<B>) -> Result<Vec<V>, B::Error> {
        let _ = data;
        Err(B::error_unsupported("download_buffer"))
    }

    /// Downloads a texture to the CPU.
    ///
    /// Performance: This method should prioritize successful downloads over high performance.
    unsafe fn download_texture<P: PixelType<B>, const N: usize>(
        &mut self,
        data: &StoredData<B>,
    ) -> Result<Texture<P, N>, B::Error> {
        let _ = data;
        Err(B::error_unsupported("download_texture"))
    }

    /// Downloads a texture to the CPU.
    ///
    /// Performance: This method should prioritize successful downloads over high performance.
    unsafe fn download_texture_array<P: PixelType<B>, const N: usize>(
        &mut self,
        data: &StoredData<B>,
    ) -> Result<TextureArray<P, N>, B::Error> {
        let _ = data;
        Err(B::error_unsupported("download_texture_array"))
    }

    /// Downloads a texture to the CPU.
    ///
    /// Performance: This method should prioritize successful downloads over high performance.
    unsafe fn download_texture_cube<P: PixelType<B>>(
        &mut self,
        data: &StoredData<B>,
    ) -> Result<TextureCube<P>, B::Error> {
        let _ = data;
        Err(B::error_unsupported("download_texture_cube"))
    }

    /// Flushes all deletes that have been queued up.
    ///
    /// Implementation: This method should either call `MemoryStorage::flush_deletes` or `MemoryStorage::extract_deletes`.
    fn flush_deletes(&mut self) -> Result<(), B::Error>;

    fn debug_value<'a>(&'a mut self, value: &StoredData<B>) -> &'a dyn std::fmt::Debug;

    /// Converts a value from the background thread to the foreground thread.
    ///
    /// In the event of an error, this method is fully responsible for freeing resources associated with the `value` parameter.
    fn background_to_foreground(
        &mut self,
        value: B::SendStoredValue,
    ) -> Result<StoredData<B>, B::Error>;

    /// Converts a value from the foreground thread to the background thread.
    ///
    /// Implementation: This method will typically call `MemoryStorage::try_take`, convert to stored value, then call
    /// `MemoryStorage::store_send`.
    fn foreground_to_background(
        &mut self,
        value: StoredData<B>,
    ) -> Result<StoredSendData<B>, B::Error>;

    fn try_index(&mut self, v: &StoredData<B>, index: usize) -> Result<StoredData<B>, B::Error>;

    fn try_slice<Bounds: std::ops::RangeBounds<usize>>(
        &mut self,
        v: &StoredData<B>,
        range: Bounds,
    ) -> Result<StoredData<B>, B::Error>;

    fn len(&mut self, v: &StoredData<B>) -> usize;
}

pub trait BackendSendDeviceInstance<B: Backend>: Send + 'static {
    fn load_buffer_data(
        &mut self,
        data: BufferData<'_>,
        options: B::LoadOptions,
    ) -> Result<StoredSendData<B>, B::Error>;

    fn load_texture_data<P: PixelType<B>>(
        &mut self,
        data: TextureData<'_, P>,
        options: B::LoadOptions,
    ) -> Result<StoredSendData<B>, B::Error>;

    fn load_sampler(
        &mut self,
        sampler: Sampler,
        options: B::LoadOptions,
    ) -> Result<StoredSendData<B>, B::Error>;

    fn load_program(
        &mut self,
        program: &ProgramRef,
        options: B::LoadOptions,
    ) -> Result<StoredSendData<B>, B::Error>;

    fn load_render_ranges(
        &mut self,
        ranges: RenderRanges,
        options: B::LoadOptions,
    ) -> Result<StoredSendData<B>, B::Error>;

    fn load_compute_workgroup(
        &mut self,
        workgroup: ComputeWorkgroup,
        options: B::LoadOptions,
    ) -> Result<StoredSendData<B>, B::Error>;
}

pub enum BackendSession<'session, B: Backend> {
    Compute(&'session mut B::ComputePassInstance),
    Render(&'session mut B::RenderPassInstance),
}

pub trait BackendPipelineBuilder<'ctxt, B: Backend>: Sized {
    fn internal_new(
        device: Device<B>,
        surface: &'ctxt mut B::SurfaceInstance,
        frame: &'ctxt mut B::FrameInstance,
        session: BackendSession<'ctxt, B>,
        dyn_params: &'ctxt [(StoredData<B>, ParameterAccessMode)],
        for_code_gen: bool,
    ) -> Result<Self, B::Error>;

    fn input_write(&mut self, input: impl Into<StoredData<B>>) -> Result<(), B::Error>;

    fn input(&mut self, input: impl Into<StoredData<B>>) -> Result<(), B::Error>;

    fn load_input<T: DeviceLoad<B>>(&mut self, input: &T) -> Result<(), B::Error>;

    fn load_array_input<T: DeviceLoad<B>>(&mut self, input: &[T]) -> Result<(), B::Error>;

    fn backface_culling(&mut self, enable: bool) -> &mut Self;
    fn frontface_culling(&mut self, enable: bool) -> &mut Self;

    fn depth_test(&mut self, depth_test: bool, depth_write: bool) -> &mut Self;

    fn alpha_blending(&mut self, enable: bool) -> &mut Self;

    fn internal_compile_pipeline(self) -> Result<B::CompiledPipeline, B::Error>;

    /// Generates sample code for the pipeline.
    ///
    /// Implementation: The expectation is that this method will generate as much code as possible for the pipeline inputs.
    /// Be aware that the configuration is likely to be incomplete (especially regarding any shader code).
    fn internal_generate_sample_code(self) -> (String, Option<B::Error>);
}

pub trait BackendFrameInstance<B: Backend> {
    /// Returns the default color render target for the current frame.
    ///
    /// Implementation: This method should return the same result for all calls within the same frame.
    fn render_target(
        &mut self,
        device: &mut B::DeviceInstance,
        surface: &mut B::SurfaceInstance,
    ) -> Result<Stored<B, Texture<Srgba, 2>>, B::Error>;

    /// Returns the active texture render target for the given name. This will create a new texture if it does not exist.
    ///
    /// Implementation: This method should return the same result for all calls within the same frame and name.
    fn named_target<P: PixelType<B>>(
        &mut self,
        device: &mut B::DeviceInstance,
        surface: &mut B::SurfaceInstance,
        name: &str,
        usage: B::LoadOptions,
    ) -> Result<Stored<B, Texture<P, 2>>, B::Error>;

    /// Discards all memory associated with named targets. For example, if multiple backing textures are used for a named target,
    /// this method should discard all of them (assuming the GPU is finished using them).
    ///
    /// The intention is that this method will only be called when the named targets are no longer needed, such as when switching
    /// from a loading screen to the main screen.
    fn discard_named_targets(
        &mut self,
        device: &mut B::DeviceInstance,
        surface: &mut B::SurfaceInstance,
    ) -> Result<(), B::Error>;
}
