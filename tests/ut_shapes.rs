use pgfx::shapes::*;

#[test]
fn ut_cube() {
    let (vertices, _) = Cube::new()
        .with_size(2.0)
        .with_offset([1.0, 2.0, 3.0])
        .build();

    let mut min_values = [f32::INFINITY; 3];
    let mut max_values = [f32::NEG_INFINITY; 3];
    for v in vertices.iter() {
        for i in 0..3 {
            min_values[i] = min_values[i].min(v.pos[i]);
            max_values[i] = max_values[i].max(v.pos[i]);
        }
    }

    assert_eq!(vertices.len(), 24);
    assert_eq!(min_values, [0.0, 1.0, 2.0]);
    assert_eq!(max_values, [2.0, 3.0, 4.0]);
}
