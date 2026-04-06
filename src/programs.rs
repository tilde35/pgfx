use crate::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Program {
    pub name: Option<String>,
    pub program_type: ProgramType,
    pub source: ProgramCode,
    pub entry_point: Option<String>,
    pub defines: Vec<(String, String)>,
}
impl Program {
    fn create_defines_vec<'a>(&'a self) -> Vec<(&'a str, &'a str)> {
        self.defines
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
    fn as_program_ref<'container: 'a, 'a>(
        &'a self,
        defines: &'container [(&'a str, &'a str)],
    ) -> ProgramRef<'a> {
        ProgramRef {
            name: self.name.as_deref(),
            program_type: self.program_type,
            source: match &self.source {
                ProgramCode::Text(code) => ProgramCodeRef::Text(code),
                ProgramCode::Compiled(code) => ProgramCodeRef::Compiled(code),
            },
            entry_point: self.entry_point.as_deref(),
            defines,
        }
    }
}
impl<'a> From<ProgramRef<'a>> for Program {
    fn from(pr: ProgramRef<'a>) -> Self {
        pr.as_program()
    }
}
impl<'r, 'a> From<&'r ProgramRef<'a>> for Program {
    fn from(pr: &'r ProgramRef<'a>) -> Self {
        pr.as_program()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProgramRef<'a> {
    pub name: Option<&'a str>,
    pub program_type: ProgramType,
    pub source: ProgramCodeRef<'a>,
    pub entry_point: Option<&'a str>,
    pub defines: &'a [(&'a str, &'a str)],
}
impl<'a> ProgramRef<'a> {
    pub fn new(name: &'a str, source_code: &'a str) -> Self {
        Self {
            name: if name.is_empty() { None } else { Some(name) },
            program_type: ProgramType::Unspecified,
            source: ProgramCodeRef::Text(source_code),
            entry_point: None,
            defines: &[],
        }
    }

    pub fn with_entry(&self, program_type: ProgramType, entry_point: &'a str) -> Self {
        let mut v = self.clone();
        v.program_type = program_type;
        v.entry_point = Some(entry_point);
        v
    }

    pub fn with_vertex_entry(&self, entry_point: &'a str) -> Self {
        self.with_entry(ProgramType::Vertex, entry_point)
    }
    pub fn with_fragment_entry(&self, entry_point: &'a str) -> Self {
        self.with_entry(ProgramType::Fragment, entry_point)
    }
    pub fn with_compute_entry(&self, entry_point: &'a str) -> Self {
        self.with_entry(ProgramType::Compute, entry_point)
    }

    pub fn with_defines(&self, defines: &'a [(&'a str, &'a str)]) -> Self {
        let mut v = self.clone();
        v.defines = defines;
        v
    }

    pub fn as_program(&self) -> Program {
        Program {
            name: self.name.map(String::from),
            program_type: self.program_type,
            source: match &self.source {
                ProgramCodeRef::Text(code) => ProgramCode::Text(code.to_string()),
                ProgramCodeRef::Compiled(code) => ProgramCode::Compiled(code.to_vec()),
            },
            entry_point: self.entry_point.map(String::from),
            defines: self
                .defines
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProgramCode {
    Text(String),
    Compiled(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProgramCodeRef<'a> {
    Text(&'a str),
    Compiled(&'a [u8]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProgramType {
    Vertex,
    Fragment,
    Compute,
    Unspecified,
}

impl<B: Backend> DeviceLoad<B> for Program {
    type LoadAs = Program;

    fn load(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            let defines = data[0].create_defines_vec();
            let program_ref = data[0].as_program_ref(&defines);
            let result = d.load_program(&program_ref, options)?;
            unsafe { Ok(result.unchecked_cast()) }
        }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            let defines = data[0].create_defines_vec();
            let program_ref = data[0].as_program_ref(&defines);
            d.reload_program(&program_ref, dest.stored_data())
        }
    }

    fn load_sendable(
        d: &mut <B as Backend>::SendDeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            let defines = data[0].create_defines_vec();
            let program_ref = data[0].as_program_ref(&defines);
            let result = d.load_program(&program_ref, options)?;
            unsafe { Ok(result.unchecked_cast()) }
        }
    }
}

impl<'a, B: Backend> DeviceLoad<B> for ProgramRef<'a> {
    type LoadAs = Program;

    fn load(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<Stored<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            let result = d.load_program(&data[0], options)?;
            unsafe { Ok(result.unchecked_cast()) }
        }
    }

    fn reload(
        d: &mut <B as Backend>::DeviceInstance,
        data: &[Self],
        dest: &Stored<B, Self::LoadAs>,
    ) -> Result<(), <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            d.reload_program(&data[0], dest.stored_data())
        }
    }

    fn load_sendable(
        d: &mut <B as Backend>::SendDeviceInstance,
        data: &[Self],
        options: <B as Backend>::LoadOptions,
    ) -> Result<StoredSend<B, Self::LoadAs>, <B as Backend>::Error> {
        if data.len() != 1 {
            Err(B::error_single_value_only())
        } else {
            let result = d.load_program(&data[0], options)?;
            unsafe { Ok(result.unchecked_cast()) }
        }
    }
}
