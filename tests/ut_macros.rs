pgfx::vertex_struct!(
    pub struct MyVertex {
        pub pos: [f32; 3],
    }
);

pgfx::instance_struct!(
    pub struct MyInstanceVertex {
        pub pos: [f32; 3],
    }
);

pgfx::uniform_struct!(
    pub struct MyUniform {
        pub light_color: [f32; 4],
    }
);

pgfx::storage_struct!(
    pub struct MyStorage {
        pub light_color: [f32; 4],
    }
);

#[derive(Default)]
pub struct AnotherVertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}
pgfx::impl_data_struct!(AnotherVertex, pos, uv);
pgfx::impl_vertex!(AnotherVertex);

#[derive(Default)]
pub struct AnotherInstanceVertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}
pgfx::impl_data_struct!(AnotherInstanceVertex, pos, uv);
pgfx::impl_instance!(AnotherInstanceVertex);

#[derive(Default)]
pub struct AnotherUniform {
    pub light_color: [f32; 4],
    pub uv: [f32; 2],
}
pgfx::impl_data_struct!(AnotherUniform, light_color, uv);
pgfx::impl_uniform!(AnotherUniform);

#[derive(Default)]
pub struct AnotherStorage {
    pub light_color: [f32; 4],
    pub uv: [f32; 2],
}
pgfx::impl_data_struct!(AnotherStorage, light_color, uv);
pgfx::impl_storage!(AnotherStorage);

pub struct MyI32(pub i32);
pub struct MyF32(pub f32);
pgfx::data_value_mapping!(MyI32 as i32, MyF32 as f32,);

#[test]
fn test_macros() {
    // This is a compilation check, not a runtime test.
    let _ = MyVertex::default();
    let _ = MyInstanceVertex::default();
    let _ = MyUniform::default();
    let _ = MyStorage::default();

    let _ = AnotherVertex::default();
    let _ = AnotherInstanceVertex::default();
    let _ = AnotherUniform::default();
    let _ = AnotherStorage::default();
}
