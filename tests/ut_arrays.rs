use pgfx::*;

storage_struct!(
    pub struct Entry {
        pub example: [f32; 3],
    }
);

storage_struct!(
    pub struct Parent {
        pub entries: Array<Entry, 3>,
    }
);

#[test]
fn test_nested_structs() {
    let layout = Parent::device_struct_layout();

    let layout_fields = layout
        .fields
        .iter()
        .map(|f| f.field_name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    assert_eq!(
        layout_fields,
        "entries<3>[0].example, entries<3>[1].example, entries<3>[2].example"
    );
}
