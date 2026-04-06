use crate::data_value_mapping;
use glam::*;

data_value_mapping!(
    Vec2 as [f32; 2],
    Vec3 as [f32; 3],
    Vec4 as [f32; 4],
    Mat2 as [[f32; 2]; 2],
    Mat3 as [[f32; 3]; 3],
    Mat4 as [[f32; 4]; 4],
    Quat as [f32; 4],
    DVec2 as [f64; 2],
    DVec3 as [f64; 3],
    DVec4 as [f64; 4],
    DMat2 as [[f64; 2]; 2],
    DMat3 as [[f64; 3]; 3],
    DMat4 as [[f64; 4]; 4],
    DQuat as [f64; 4],
    IVec2 as [i32; 2],
    IVec3 as [i32; 3],
    IVec4 as [i32; 4],
    UVec2 as [u32; 2],
    UVec3 as [u32; 3],
    UVec4 as [u32; 4],
    BVec2 as [bool; 2],
    BVec3 as [bool; 3],
    BVec4 as [bool; 4],
);
