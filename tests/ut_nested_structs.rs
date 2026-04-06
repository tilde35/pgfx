use pgfx::*;

uniform_struct!(
    pub struct MatrixData {
        pub model: [[f32; 4]; 4],
        pub view: [[f32; 4]; 4],
        pub projection: [[f32; 4]; 4],
    }
);

uniform_struct!(
    pub struct SceneData {
        pub matrix_data: MatrixData,
        pub time: f32,
    }
);

#[test]
fn test_nested_structs() {
    let layout = SceneData::device_struct_layout();

    let layout_fields = layout
        .fields
        .iter()
        .map(|f| f.field_name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    assert_eq!(layout_fields, "matrix_data.model, matrix_data.view, matrix_data.projection, time");
}
