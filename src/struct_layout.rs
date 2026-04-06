use crate::*;

pub trait DataStruct {
    fn device_struct_layout() -> &'static StructLayout;
}

#[derive(Debug)]
pub struct StructLayout {
    pub name: String,
    pub size: usize,
    pub alignment: usize,
    pub fields: Vec<StructField>,
}
impl StructLayout {
    /// Constructs the device's struct layout using a given value. Example usage:
    ///
    /// ```ignore
    /// StructLayout::from_value(&MyVertex::default(), "MyVertex")
    ///     .add_field(|v| &v.position, "position")
    ///     .add_field(|v| &v.color, "color")
    ///     .build();
    /// ```
    pub fn from_value<'a, T>(value: &'a T, name: &str) -> StructLayoutHelper<'a, T> {
        StructLayoutHelper {
            ptr: value,
            name: name.to_string(),
            vertex_size: std::mem::size_of::<T>(),
            fields: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct StructField {
    pub field_name: String,
    pub field_type: ValueType,
    pub offset: usize,
    pub size: usize,
}

pub struct StructLayoutHelper<'a, T> {
    ptr: &'a T,
    name: String,
    vertex_size: usize,
    fields: Vec<StructField>,
}
impl<'a, T> StructLayoutHelper<'a, T> {
    pub fn add_field<F: DeviceValueTypeOrStruct>(
        &mut self,
        field: impl Fn(&T) -> &F,
        name: &str,
    ) -> &mut Self {
        F::add_data_to_struct_layout(self, field, name);
        self
    }
    pub fn add_field_of_type<F>(
        &mut self,
        field: impl Fn(&T) -> &F,
        name: &str,
        data_type: ValueType,
    ) -> &mut Self {
        let f = field(self.ptr);
        let size = std::mem::size_of::<F>();

        // Determine where the field is located inside the struct
        let offset = ((f as *const _) as isize) - ((self.ptr as *const _) as isize);
        // Check that the field is within the bounds of the struct. This should always be the case, if it's not, then it
        // is likely one of the following situations:
        // 1. The field is not actually part of the struct
        // 2. The field is referencing heap-allocated data (which would not be possible to store in a vertex buffer)
        assert!(
            offset >= 0 && (offset as usize) + size <= self.vertex_size,
            "Field offset is out of bounds"
        );

        self.fields.push(StructField {
            field_name: name.to_string(),
            field_type: data_type,
            offset: offset as usize,
            size,
        });
        self
    }
    pub fn add_value_field<F: DeviceValueType>(
        &mut self,
        field: impl Fn(&T) -> &F,
        name: &str,
    ) -> &mut Self {
        let data_type = F::device_value_type();
        self.add_field_of_type(field, name, data_type)
    }
    pub fn add_struct_field<F: DataStruct>(
        &mut self,
        field: impl Fn(&T) -> &F,
        name: &str,
    ) -> &mut Self {
        let struct_layout = F::device_struct_layout();

        let f = field(self.ptr);
        let size = std::mem::size_of::<F>();

        // Determine where the field is located inside the struct
        let struct_offset = ((f as *const _) as isize) - ((self.ptr as *const _) as isize);
        // Check that the field is within the bounds of the struct. This should always be the case, if it's not, then it
        // is likely one of the following situations:
        // 1. The field is not actually part of the struct
        // 2. The field is referencing heap-allocated data (which would not be possible to store in a vertex buffer)
        assert!(
            struct_offset >= 0 && (struct_offset as usize) + size <= self.vertex_size,
            "Field offset is out of bounds"
        );
        let struct_offset = struct_offset as usize;

        for f in struct_layout.fields.iter() {
            let field_name = format!("{}.{}", name, f.field_name);

            self.fields.push(StructField {
                field_name: field_name.clone(),
                field_type: f.field_type.clone(),
                offset: struct_offset + f.offset,
                size: f.size,
            });
        }
        self
    }

    pub fn build(&mut self) -> StructLayout {
        StructLayout {
            name: std::mem::take(&mut self.name),
            size: self.vertex_size,
            alignment: std::mem::align_of::<T>(),
            fields: std::mem::take(&mut self.fields),
        }
    }
}
