use crate::*;
use std::cell::Cell;

pub struct Pipeline<B: Backend> {
    name: String,
    internal_data: Cell<Option<B::CompiledPipeline>>,
}
impl<B: Backend> Pipeline<B> {
    pub fn new(name: impl Into<String>) -> Self {
        Pipeline {
            name: name.into(),
            internal_data: Cell::new(None),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_compiled(&self) -> bool {
        let tmp = self.internal_data.take();
        let result = tmp.is_some();
        self.internal_data.set(tmp);
        result
    }

    pub fn reset(&self) {
        self.internal_data.take();
    }

    pub(crate) fn take_internal_data(&self) -> Option<B::CompiledPipeline> {
        self.internal_data.take()
    }
    pub(crate) fn set_internal_data(&self, data: B::CompiledPipeline) {
        self.internal_data.set(Some(data));
    }
}

pub enum PipelineSession<'data, 's, B: Backend> {
    Compute(&'s mut ComputePass<'data, B>),
    Render(&'s mut RenderPass<'data, B>),
}

pub struct ActivePipeline<'data, 's, B: Backend> {
    device: &'s Device<B>,
    session: PipelineSession<'data, 's, B>,
    pipeline: &'s Pipeline<B>,
    dynamic_inputs: Vec<(StoredData<B>, ParameterAccessMode)>,
    failed: Option<B::Error>,
}
impl<'data, 's, B: Backend> ActivePipeline<'data, 's, B> {
    pub(crate) fn new_compute(
        device: &'s Device<B>,
        session: &'s mut ComputePass<'data, B>,
        pipeline: &'s Pipeline<B>,
    ) -> Self {
        ActivePipeline {
            device,
            session: PipelineSession::Compute(session),
            pipeline,
            dynamic_inputs: Vec::new(),
            failed: None,
        }
    }
    pub(crate) fn new_render(
        device: &'s Device<B>,
        session: &'s mut RenderPass<'data, B>,
        pipeline: &'s Pipeline<B>,
    ) -> Self {
        ActivePipeline {
            session: PipelineSession::Render(session),
            device,
            pipeline,
            dynamic_inputs: Vec::new(),
            failed: None,
        }
    }

    #[must_use]
    pub fn input_write(mut self, input: impl Into<StoredData<B>>) -> Self {
        self.dynamic_inputs
            .push((input.into(), ParameterAccessMode::Write));
        self
    }

    #[must_use]
    pub fn input(mut self, input: impl Into<StoredData<B>>) -> Self {
        self.dynamic_inputs
            .push((input.into(), ParameterAccessMode::Read));
        self
    }

    #[must_use]
    pub fn load_input<T: DeviceLoad<B>>(self, input: &T) -> Self {
        self.load_array_input(std::slice::from_ref(input))
    }

    #[must_use]
    pub fn load_array_input<T: DeviceLoad<B>>(mut self, input: &[T]) -> Self {
        if self.failed.is_none() {
            let opts = B::default_parameter_load_options();
            let d = self.device.internal(|d| T::load(d, input, opts));

            match d {
                Ok(d) => {
                    self.dynamic_inputs
                        .push((d.into_stored_data(), ParameterAccessMode::Read));
                }
                Err(e) => {
                    self.failed = Some(e);
                }
            }
        }
        self
    }

    fn compile(
        &mut self,
        cfg: impl FnOnce(&mut B::PipelineBuilder<'_>) -> Result<(), B::Error>,
    ) -> Result<(), B::Error> {
        let (surface, frame, session) = match &mut self.session {
            PipelineSession::Compute(compute_pass) => (
                &mut *compute_pass.surface,
                &mut *compute_pass.frame,
                BackendSession::Compute(compute_pass.pass.as_mut().unwrap()),
            ),
            PipelineSession::Render(render_pass) => (
                &mut *render_pass.surface,
                &mut *render_pass.frame,
                BackendSession::Render(render_pass.pass.as_mut().unwrap()),
            ),
        };

        let mut builder = <B::PipelineBuilder<'_>>::internal_new(
            self.device.clone(),
            surface,
            frame,
            session,
            &self.dynamic_inputs,
            false,
        )?;

        cfg(&mut builder)?;

        let result = builder.internal_compile_pipeline()?;
        self.pipeline.set_internal_data(result);

        Ok(())
    }

    pub fn execute(
        mut self,
        cfg: impl FnOnce(&mut B::PipelineBuilder<'_>) -> Result<(), B::Error>,
    ) -> Result<(), B::Error> {
        if let Some(e) = self.failed.take() {
            // If there was a failure, return it
            return Err(e);
        }

        if let Some(mut p) = self.pipeline.take_internal_data() {
            // If the pipeline is already compiled, execute it
            let result = match &mut self.session {
                PipelineSession::Compute(compute_pass) => self.device.internal(|d| {
                    B::run_compute_pipeline(
                        d,
                        compute_pass.surface,
                        compute_pass.frame,
                        compute_pass.pass.as_mut().unwrap(),
                        &mut p,
                        &self.dynamic_inputs,
                    )
                }),
                PipelineSession::Render(render_pass) => self.device.internal(|d| {
                    B::run_render_pipeline(
                        d,
                        render_pass.surface,
                        render_pass.frame,
                        render_pass.pass.as_mut().unwrap(),
                        &mut p,
                        &self.dynamic_inputs,
                    )
                }),
            };

            // Restore the internal data
            self.pipeline.set_internal_data(p);

            result
        } else {
            self.compile(cfg)?;

            let mut p = self.pipeline.take_internal_data().unwrap();
            let result = match &mut self.session {
                PipelineSession::Compute(compute_pass) => self.device.internal(|d| {
                    B::run_compute_pipeline(
                        d,
                        compute_pass.surface,
                        compute_pass.frame,
                        compute_pass.pass.as_mut().unwrap(),
                        &mut p,
                        &self.dynamic_inputs,
                    )
                }),
                PipelineSession::Render(render_pass) => self.device.internal(|d| {
                    B::run_render_pipeline(
                        d,
                        render_pass.surface,
                        render_pass.frame,
                        render_pass.pass.as_mut().unwrap(),
                        &mut p,
                        &self.dynamic_inputs,
                    )
                }),
            };

            // Restore the internal data
            self.pipeline.set_internal_data(p);

            result
        }
    }

    pub fn build_only(
        mut self,
        cfg: impl FnOnce(&mut B::PipelineBuilder<'_>) -> Result<(), B::Error>,
    ) -> Result<bool, B::Error> {
        if let Some(e) = self.failed.take() {
            // If there was a failure, return it
            return Err(e);
        }

        if self.pipeline.is_compiled() {
            // Already built, nothing to do
            Ok(false)
        } else {
            // Compile, but don't execute
            self.compile(cfg)?;
            Ok(true)
        }
    }

    pub fn generate_sample_code(
        mut self,
        cfg: impl FnOnce(&mut B::PipelineBuilder<'_>) -> Result<(), B::Error>,
    ) -> (String, Option<B::Error>) {
        let (surface, frame, session) = match &mut self.session {
            PipelineSession::Compute(compute_pass) => (
                &mut *compute_pass.surface,
                &mut *compute_pass.frame,
                BackendSession::Compute(compute_pass.pass.as_mut().unwrap()),
            ),
            PipelineSession::Render(render_pass) => (
                &mut *render_pass.surface,
                &mut *render_pass.frame,
                BackendSession::Render(render_pass.pass.as_mut().unwrap()),
            ),
        };

        let builder = <B::PipelineBuilder<'_>>::internal_new(
            self.device.clone(),
            surface,
            frame,
            session,
            &self.dynamic_inputs,
            false,
        );

        let mut builder = match builder {
            Ok(b) => b,
            Err(e) => return (String::new(), Some(e)),
        };

        let cfg_result = cfg(&mut builder);
        let (src, mut err) = builder.internal_generate_sample_code();

        // Select the most relevant error
        if err.is_none() {
            if let Err(e) = cfg_result {
                err = Some(e);
            }
            if let Some(e) = self.failed.take() {
                err = Some(e);
            }
        }

        (src, err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterAccessMode {
    Read,
    Write,
}
impl Default for ParameterAccessMode {
    fn default() -> Self {
        ParameterAccessMode::Read
    }
}
impl ParameterAccessMode {
    pub fn is_read(&self) -> bool {
        match self {
            ParameterAccessMode::Read => true,
            ParameterAccessMode::Write => false,
        }
    }
    pub fn is_write(&self) -> bool {
        match self {
            ParameterAccessMode::Read => false,
            ParameterAccessMode::Write => true,
        }
    }
}
