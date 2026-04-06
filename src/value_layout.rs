use crate::*;

pub trait DeviceValueType {
    fn device_value_type() -> ValueType;
}

pub trait DeviceValueTypeOrStruct {
    fn add_data_to_struct_layout<'a, T>(
        layout: &mut StructLayoutHelper<'a, T>,
        field: impl Fn(&T) -> &Self,
        name: &str,
    );
}
impl<V: DeviceValueType> DeviceValueTypeOrStruct for V {
    fn add_data_to_struct_layout<'a, T>(
        layout: &mut StructLayoutHelper<'a, T>,
        field: impl Fn(&T) -> &Self,
        name: &str,
    ) {
        layout.add_value_field(field, name);
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ValueType {
    V1(PrimitiveType),
    V2(PrimitiveType),
    V3(PrimitiveType),
    V4(PrimitiveType),
    M2x2(PrimitiveType),
    M3x3(PrimitiveType),
    M4x4(PrimitiveType),
    Custom(&'static CustomValueType),
}
impl ValueType {
    pub fn size(&self) -> usize {
        match self {
            ValueType::V1(t) => t.size(),
            ValueType::V2(t) => t.size() * 2,
            ValueType::V3(t) => t.size() * 3,
            ValueType::V4(t) => t.size() * 4,
            ValueType::M2x2(t) => t.size() * 4,
            ValueType::M3x3(t) => t.size() * 9,
            ValueType::M4x4(t) => t.size() * 16,
            ValueType::Custom(c) => c.size,
        }
    }
    pub fn alignment(&self) -> usize {
        match self {
            ValueType::V1(t) => t.alignment(),
            ValueType::V2(t) => t.alignment(),
            ValueType::V3(t) => t.alignment(),
            ValueType::V4(t) => t.alignment(),
            ValueType::M2x2(t) => t.alignment(),
            ValueType::M3x3(t) => t.alignment(),
            ValueType::M4x4(t) => t.alignment(),
            ValueType::Custom(c) => c.alignment,
        }
    }
}
impl ValueType {
    pub fn type_name(&self) -> String {
        match self {
            ValueType::V1(primitive_type) => format!("{}", primitive_type.type_name()),
            ValueType::V2(primitive_type) => format!("[{}; 2]", primitive_type.type_name()),
            ValueType::V3(primitive_type) => format!("[{}; 3]", primitive_type.type_name()),
            ValueType::V4(primitive_type) => format!("[{}; 4]", primitive_type.type_name()),
            ValueType::M2x2(primitive_type) => format!("[[{}; 2]; 2]", primitive_type.type_name()),
            ValueType::M3x3(primitive_type) => format!("[[{}; 3]; 3]", primitive_type.type_name()),
            ValueType::M4x4(primitive_type) => format!("[[{}; 4]; 4]", primitive_type.type_name()),
            ValueType::Custom(custom_value_type) => format!("{}", custom_value_type.name),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PrimitiveType {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}
impl PrimitiveType {
    pub fn size(&self) -> usize {
        match self {
            PrimitiveType::Bool => 1,
            PrimitiveType::I8 => 1,
            PrimitiveType::U8 => 1,
            PrimitiveType::I16 => 2,
            PrimitiveType::U16 => 2,
            PrimitiveType::I32 => 4,
            PrimitiveType::U32 => 4,
            PrimitiveType::I64 => 8,
            PrimitiveType::U64 => 8,
            PrimitiveType::F32 => 4,
            PrimitiveType::F64 => 8,
        }
    }
    pub fn alignment(&self) -> usize {
        match self {
            PrimitiveType::Bool => 1,
            PrimitiveType::I8 => 1,
            PrimitiveType::U8 => 1,
            PrimitiveType::I16 => 2,
            PrimitiveType::U16 => 2,
            PrimitiveType::I32 => 4,
            PrimitiveType::U32 => 4,
            PrimitiveType::I64 => 8,
            PrimitiveType::U64 => 8,
            PrimitiveType::F32 => 4,
            PrimitiveType::F64 => 8,
        }
    }
}
impl PrimitiveType {
    pub fn type_name(&self) -> &'static str {
        match self {
            PrimitiveType::Bool => "bool",
            PrimitiveType::I8 => "i8",
            PrimitiveType::U8 => "u8",
            PrimitiveType::I16 => "i16",
            PrimitiveType::U16 => "u16",
            PrimitiveType::I32 => "i32",
            PrimitiveType::U32 => "u32",
            PrimitiveType::I64 => "i64",
            PrimitiveType::U64 => "u64",
            PrimitiveType::F32 => "f32",
            PrimitiveType::F64 => "f64",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CustomValueType {
    pub namespace: &'static str,
    pub name: &'static str,
    pub size: usize,
    pub alignment: usize,
}

#[macro_export]
macro_rules! impl_primitive_types {
    ( $($t:ty as PrimitiveType::$v:ident),* $(,)? ) => {
        $(
            impl $crate::DeviceValueType for $t {
                fn device_value_type() -> $crate::ValueType {
                    $crate::ValueType::V1($crate::PrimitiveType::$v)
                }
            }
            impl $crate::DeviceValueType for [$t; 1] {
                fn device_value_type() -> $crate::ValueType {
                    $crate::ValueType::V1($crate::PrimitiveType::$v)
                }
            }
            impl $crate::DeviceValueType for [$t; 2] {
                fn device_value_type() -> $crate::ValueType {
                    $crate::ValueType::V2($crate::PrimitiveType::$v)
                }
            }
            impl $crate::DeviceValueType for [$t; 3] {
                fn device_value_type() -> $crate::ValueType {
                    $crate::ValueType::V3($crate::PrimitiveType::$v)
                }
            }
            impl $crate::DeviceValueType for [$t; 4] {
                fn device_value_type() -> $crate::ValueType {
                    $crate::ValueType::V4($crate::PrimitiveType::$v)
                }
            }
            impl $crate::DeviceValueType for [[$t; 2]; 2] {
                fn device_value_type() -> $crate::ValueType {
                    $crate::ValueType::M2x2($crate::PrimitiveType::$v)
                }
            }
            impl $crate::DeviceValueType for [[$t; 3]; 3] {
                fn device_value_type() -> $crate::ValueType {
                    $crate::ValueType::M3x3($crate::PrimitiveType::$v)
                }
            }
            impl $crate::DeviceValueType for [[$t; 4]; 4] {
                fn device_value_type() -> $crate::ValueType {
                    $crate::ValueType::M4x4($crate::PrimitiveType::$v)
                }
            }
        )*
    };
}
impl_primitive_types!(
    bool as PrimitiveType::Bool,
    i8 as PrimitiveType::I8,
    u8 as PrimitiveType::U8,
    i16 as PrimitiveType::I16,
    u16 as PrimitiveType::U16,
    i32 as PrimitiveType::I32,
    u32 as PrimitiveType::U32,
    i64 as PrimitiveType::I64,
    u64 as PrimitiveType::U64,
    f32 as PrimitiveType::F32,
    f64 as PrimitiveType::F64,
);
