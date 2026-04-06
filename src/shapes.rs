use crate::*;

vertex_struct!(
    pub struct ShapeVertex {
        pub pos: [f32; 3],
        pub normal: [f32; 3],
        pub uv: [f32; 2],
        pub image_index: u32,
    }
);

/// A simple 2D grid in the XY plane, with normals facing +Z.
#[derive(Clone, Debug)]
pub struct Grid {
    size: [f32; 2],
    offset: [f32; 2],
    divisions: [u32; 2],
}
impl Grid {
    pub fn new() -> Self {
        Self {
            size: [1.0, 1.0],
            offset: [0.0, 0.0],
            divisions: [1, 1],
        }
    }
    pub fn with_size(self, size: [f32; 2]) -> Self {
        Self {
            size,
            offset: self.offset,
            divisions: self.divisions,
        }
    }
    pub fn with_offset(self, offset: [f32; 2]) -> Self {
        Self {
            size: self.size,
            offset,
            divisions: self.divisions,
        }
    }
    pub fn with_divisions(self, divisions: [u32; 2]) -> Self {
        Self {
            size: self.size,
            offset: self.offset,
            divisions,
        }
    }

    pub fn vertex_iter(&self) -> impl Iterator<Item = ShapeVertex> {
        let step = [
            self.size[0] / self.divisions[0] as f32,
            self.size[1] / self.divisions[1] as f32,
        ];
        (0..=self.divisions[0]).flat_map(move |i| {
            (0..=self.divisions[1]).map(move |j| ShapeVertex {
                pos: [
                    i as f32 * step[0] + self.offset[0],
                    j as f32 * step[1] + self.offset[1],
                    0.0,
                ],
                normal: [0.0, 0.0, 1.0],
                uv: [
                    i as f32 / self.divisions[0] as f32,
                    j as f32 / self.divisions[1] as f32,
                ],
                image_index: 0,
            })
        })
    }

    pub fn index_iter(&self, offset: u32) -> impl Iterator<Item = u32> {
        (0..self.divisions[0])
            .flat_map(move |i| {
                (0..self.divisions[1]).map(move |j| {
                    let bottom_left = i * (self.divisions[1] + 1) + j + offset;
                    let bottom_right = bottom_left + 1;
                    let top_left = bottom_left + self.divisions[1] + 1;
                    let top_right = top_left + 1;
                    [
                        // First triangle
                        top_left,
                        bottom_left,
                        bottom_right,
                        // Second triangle
                        top_left,
                        bottom_right,
                        top_right,
                    ]
                })
            })
            .flatten()
    }

    pub fn build(&self) -> (Vec<ShapeVertex>, Vec<u32>) {
        let vertices = self.vertex_iter().collect::<Vec<_>>();
        let indexes = self.index_iter(0).collect::<Vec<_>>();
        (vertices, indexes)
    }
}

#[derive(Clone, Debug)]
pub struct UvSphere {
    radius: f32,
    divisions: [u32; 2],
}
impl UvSphere {
    pub fn new() -> Self {
        Self {
            radius: 0.5,
            divisions: [16, 16],
        }
    }
    pub fn with_radius(self, radius: f32) -> Self {
        Self {
            radius,
            divisions: self.divisions,
        }
    }
    pub fn with_divisions(self, divisions: [u32; 2]) -> Self {
        Self {
            radius: self.radius,
            divisions,
        }
    }

    pub fn build(&self) -> (Vec<ShapeVertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indexes = Vec::new();

        for i in 0..=self.divisions[0] {
            let theta = i as f32 / self.divisions[0] as f32 * std::f32::consts::PI;
            for j in 0..=self.divisions[1] {
                let phi = j as f32 / self.divisions[1] as f32 * 2.0 * std::f32::consts::PI;
                let x = self.radius * theta.sin() * phi.cos();
                let y = self.radius * theta.sin() * phi.sin();
                let z = self.radius * theta.cos();
                vertices.push(ShapeVertex {
                    pos: [x, y, z],
                    normal: [x / self.radius, y / self.radius, z / self.radius],
                    uv: [
                        j as f32 / self.divisions[1] as f32,
                        i as f32 / self.divisions[0] as f32,
                    ],
                    image_index: 0,
                });
            }
        }

        for i in 0..self.divisions[0] {
            for j in 0..self.divisions[1] {
                let top_left = i * (self.divisions[1] + 1) + j;
                let top_right = top_left + 1;
                let bottom_left = top_left + self.divisions[1] + 1;
                let bottom_right = bottom_left + 1;

                indexes.push(top_left);
                indexes.push(bottom_left);
                indexes.push(bottom_right);

                indexes.push(top_left);
                indexes.push(bottom_right);
                indexes.push(top_right);
            }
        }

        (vertices, indexes)
    }
}

#[derive(Clone, Debug)]
pub struct QuadSphere {
    radius: f32,
    divisions: u32,
}
impl QuadSphere {
    pub fn new() -> Self {
        Self {
            radius: 0.5,
            divisions: 16,
        }
    }
    pub fn with_radius(self, radius: f32) -> Self {
        Self {
            radius,
            divisions: self.divisions,
        }
    }
    pub fn with_divisions(self, divisions: u32) -> Self {
        Self {
            radius: self.radius,
            divisions,
        }
    }

    pub fn build(&self) -> (Vec<ShapeVertex>, Vec<u32>) {
        let face_count = Face::ALL.len() as u32;
        let vertex_count_per_face = (self.divisions + 1) * (self.divisions + 1);
        let index_count_per_face = self.divisions * self.divisions * 4;
        let mut vertices = Vec::with_capacity((face_count * vertex_count_per_face) as usize);
        let mut indexes = Vec::with_capacity((face_count * index_count_per_face) as usize);

        let faces = [
            (
                Face::XNeg,
                [-1.0, -1.0, -1.0],
                [0.0, 0.0, 2.0],
                [0.0, 2.0, 0.0],
            ),
            (
                Face::XPos,
                [1.0, -1.0, 1.0],
                [0.0, 0.0, -2.0],
                [0.0, 2.0, 0.0],
            ),
            (
                Face::YNeg,
                [-1.0, -1.0, -1.0],
                [2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0],
            ),
            (
                Face::YPos,
                [-1.0, 1.0, 1.0],
                [2.0, 0.0, 0.0],
                [0.0, 0.0, -2.0],
            ),
            (
                Face::ZNeg,
                [1.0, -1.0, -1.0],
                [-2.0, 0.0, 0.0],
                [0.0, 2.0, 0.0],
            ),
            (
                Face::ZPos,
                [-1.0, -1.0, 1.0],
                [2.0, 0.0, 0.0],
                [0.0, 2.0, 0.0],
            ),
        ];

        for (face, origin, u_dir, v_dir) in faces {
            let base_index = vertices.len() as u32;

            for i in 0..=self.divisions {
                let u = i as f32 / self.divisions as f32;
                for j in 0..=self.divisions {
                    let v = j as f32 / self.divisions as f32;
                    let cube_pos = [
                        origin[0] + u_dir[0] * u + v_dir[0] * v,
                        origin[1] + u_dir[1] * u + v_dir[1] * v,
                        origin[2] + u_dir[2] * u + v_dir[2] * v,
                    ];
                    let len = (cube_pos[0] * cube_pos[0]
                        + cube_pos[1] * cube_pos[1]
                        + cube_pos[2] * cube_pos[2])
                        .sqrt();
                    let normal = [cube_pos[0] / len, cube_pos[1] / len, cube_pos[2] / len];

                    vertices.push(ShapeVertex {
                        pos: [
                            normal[0] * self.radius,
                            normal[1] * self.radius,
                            normal[2] * self.radius,
                        ],
                        normal,
                        uv: [u, v],
                        image_index: face as u32,
                    });
                }
            }

            let row_len = self.divisions + 1;
            for i in 0..self.divisions {
                for j in 0..self.divisions {
                    let top_left = base_index + i * row_len + j;
                    let top_right = top_left + 1;
                    let bottom_left = top_left + row_len;
                    let bottom_right = bottom_left + 1;

                    indexes.push(top_left);
                    indexes.push(bottom_left);
                    indexes.push(bottom_right);

                    indexes.push(top_left);
                    indexes.push(bottom_right);
                    indexes.push(top_right);
                }
            }
        }

        (vertices, indexes)
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    size: f32,
    offset: [f32; 3],
}
impl Cube {
    pub fn new() -> Self {
        Self {
            size: 1.0,
            offset: [0.0, 0.0, 0.0],
        }
    }
    pub fn with_size(self, size: f32) -> Self {
        Self {
            size,
            offset: self.offset,
        }
    }
    pub fn with_offset(self, offset: [f32; 3]) -> Self {
        Self {
            size: self.size,
            offset,
        }
    }

    pub fn vertex_iter(&self) -> impl Iterator<Item = ShapeVertex> {
        let r = self.size * 0.5;
        CUBE_VERTEXES.iter().map(move |v| ShapeVertex {
            pos: [
                v.pos[0] * r + self.offset[0],
                v.pos[1] * r + self.offset[1],
                v.pos[2] * r + self.offset[2],
            ],
            normal: v.normal,
            uv: v.uv,
            image_index: v.image_index,
        })
    }
    pub fn vertex_face_iter(&self) -> impl Iterator<Item = (ShapeVertex, Face)> {
        let r = self.size * 0.5;
        CUBE_VERTEXES.iter().map(move |v| {
            (
                ShapeVertex {
                    pos: [
                        v.pos[0] * r + self.offset[0],
                        v.pos[1] * r + self.offset[1],
                        v.pos[2] * r + self.offset[2],
                    ],
                    normal: v.normal,
                    uv: v.uv,
                    image_index: v.image_index,
                },
                Face::ALL[v.image_index as usize],
            )
        })
    }
    pub fn image_faces(&self) -> &'static [Face; 6] {
        &Face::ALL
    }
    pub fn index_iter(&self, offset: u32) -> impl Iterator<Item = u32> {
        CUBE_INDEXES.iter().cloned().map(move |i| i + offset)
    }

    pub fn build(&self) -> (Vec<ShapeVertex>, Vec<u32>) {
        let vertices = self.vertex_iter().collect::<Vec<_>>();
        let indexes = self.index_iter(0).collect::<Vec<_>>();
        (vertices, indexes)
    }
}

/// An axis-aligned cube with range (-1, -1, -1) to (1, 1, 1).
#[rustfmt::skip]
const CUBE_VERTEXES: [ShapeVertex; 24] = [
    // Front (+Z)
    ShapeVertex { pos: [-1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [ 0.0,  0.0], image_index: Face::ZPos as u32 },
    ShapeVertex { pos: [ 1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [ 1.0,  0.0], image_index: Face::ZPos as u32 },
    ShapeVertex { pos: [ 1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [ 1.0,  1.0], image_index: Face::ZPos as u32 },
    ShapeVertex { pos: [-1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [ 0.0,  1.0], image_index: Face::ZPos as u32 },
    // Back (-Z)
    ShapeVertex { pos: [ 1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [ 0.0,  0.0], image_index: Face::ZNeg as u32 },
    ShapeVertex { pos: [-1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [ 1.0,  0.0], image_index: Face::ZNeg as u32 },
    ShapeVertex { pos: [-1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [ 1.0,  1.0], image_index: Face::ZNeg as u32 },
    ShapeVertex { pos: [ 1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [ 0.0,  1.0], image_index: Face::ZNeg as u32 },
    // Left (-X)
    ShapeVertex { pos: [-1.0, -1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [ 0.0,  0.0], image_index: Face::XNeg as u32 },
    ShapeVertex { pos: [-1.0, -1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [ 1.0,  0.0], image_index: Face::XNeg as u32 },
    ShapeVertex { pos: [-1.0,  1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [ 1.0,  1.0], image_index: Face::XNeg as u32 },
    ShapeVertex { pos: [-1.0,  1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [ 0.0,  1.0], image_index: Face::XNeg as u32 },
    // Right (+X)
    ShapeVertex { pos: [ 1.0, -1.0,  1.0], normal: [ 1.0,  0.0,  0.0], uv: [ 0.0,  0.0], image_index: Face::XPos as u32 },
    ShapeVertex { pos: [ 1.0, -1.0, -1.0], normal: [ 1.0,  0.0,  0.0], uv: [ 1.0,  0.0], image_index: Face::XPos as u32 },
    ShapeVertex { pos: [ 1.0,  1.0, -1.0], normal: [ 1.0,  0.0,  0.0], uv: [ 1.0,  1.0], image_index: Face::XPos as u32 },
    ShapeVertex { pos: [ 1.0,  1.0,  1.0], normal: [ 1.0,  0.0,  0.0], uv: [ 0.0,  1.0], image_index: Face::XPos as u32 },
    // Top (+Y)
    ShapeVertex { pos: [-1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0], uv: [ 0.0,  0.0], image_index: Face::YPos as u32 },
    ShapeVertex { pos: [ 1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0], uv: [ 1.0,  0.0], image_index: Face::YPos as u32 },
    ShapeVertex { pos: [ 1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0], uv: [ 1.0,  1.0], image_index: Face::YPos as u32 },
    ShapeVertex { pos: [-1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0], uv: [ 0.0,  1.0], image_index: Face::YPos as u32 },
    // Bottom (-Y)
    ShapeVertex { pos: [-1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0], uv: [ 0.0,  0.0], image_index: Face::YNeg as u32 },
    ShapeVertex { pos: [ 1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0], uv: [ 1.0,  0.0], image_index: Face::YNeg as u32 },
    ShapeVertex { pos: [ 1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0], uv: [ 1.0,  1.0], image_index: Face::YNeg as u32 },
    ShapeVertex { pos: [-1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0], uv: [ 0.0,  1.0], image_index: Face::YNeg as u32 },
];
const CUBE_INDEXES: [u32; 36] = [
    0, 1, 2, 0, 2, 3, // Front face
    4, 5, 6, 4, 6, 7, // Back face
    8, 9, 10, 8, 10, 11, // Left face
    12, 13, 14, 12, 14, 15, // Right face
    16, 17, 18, 16, 18, 19, // Top face
    20, 21, 22, 20, 22, 23, // Bottom face
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Face {
    /// The left face (assuming +Z is facing the viewer and +Y is up) with normal (-1, 0, 0).
    ///
    /// Increasing UV => (increasing Z, increasing Y)
    XNeg = 0,

    /// The right face (assuming +Z is facing the viewer and +Y is up) with normal (1, 0, 0).
    ///
    /// Increasing UV => (decreasing Z, increasing Y)
    XPos = 1,

    /// The bottom face (assuming +Z is facing the viewer and +Y is up) with normal (0, -1, 0).
    ///
    /// Increasing UV => (increasing X, increasing Z)
    YNeg = 2,

    /// The top face (assuming +Z is facing the viewer and +Y is up) with normal (0, 1, 0).
    ///
    /// Increasing UV => (increasing X, decreasing Z)
    YPos = 3,

    /// The back face (assuming +Z is facing the viewer and +Y is up) with normal (0, 0, -1).
    ///
    /// Increasing UV => (decreasing X, increasing Y)
    ZNeg = 4,

    /// The front face (assuming +Z is facing the viewer and +Y is up) with normal (0, 0, 1).
    ///
    /// Increasing UV => (increasing X, increasing Y)
    ZPos = 5,
}
impl Default for Face {
    fn default() -> Self {
        Face::XNeg
    }
}
impl Face {
    pub const ALL: [Face; 6] = [
        Face::XNeg,
        Face::XPos,
        Face::YNeg,
        Face::YPos,
        Face::ZNeg,
        Face::ZPos,
    ];

    pub fn dir(&self) -> [f32; 3] {
        match self {
            Face::XNeg => [-1.0, 0.0, 0.0],
            Face::XPos => [1.0, 0.0, 0.0],
            Face::YNeg => [0.0, -1.0, 0.0],
            Face::YPos => [0.0, 1.0, 0.0],
            Face::ZNeg => [0.0, 0.0, -1.0],
            Face::ZPos => [0.0, 0.0, 1.0],
        }
    }

    // TODO! Create a UV mapping that is continous across the faces, then -Z and +Z. Start with -X
    // pub fn uv_around_z(&self, uv: [f32; 2]) -> [f32; 2] {}
}
