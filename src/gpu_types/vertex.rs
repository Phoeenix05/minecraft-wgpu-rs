#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    // const ATTRIBS: [wgpu::VertexAttribute; 2] =
    //     wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
            // attributes: &Self::ATTRIBS,
        }
    }
}

// macro_rules! srgb {
//     ($c: expr) => {
//         (($r / 255 + 0.055) / 1.055) ^ 2.4
//     };
//     ($r: expr, $g: expr, $b: expr) => {{
//         [
//             ($r as f32 / 255 as f32 + 0.055) / 1.055 ^ 2.4 as f32,
//             ($g as f32 / 255 as f32 + 0.055) / 1.055 ^ 2.4 as f32,
//             ($b as f32 / 255 as f32 + 0.055) / 1.055 ^ 2.4 as f32
//         ]
//     }};
// }

#[rustfmt::skip]
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [1.0, 0.0] }, // A
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] }, // B
    Vertex { position: [ 0.5,  0.5, 0.0], tex_coords: [0.0, 0.0] }, // C
    Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [0.0, 0.0] }, // D
    // Vertex { position: [ 0.44147372,  0.2347359,  0.0], color: [0.0, 0.0, 1.0], }, // E
];

// #[rustfmt::skip]
// pub const VERTICES: &[Vertex] = vertex_array!(
//     [-0.0868241, 0.49240386, 0.0], [0.4131759, 0.00759614],
//     [-0.49513406, 0.06958647, 0.0], [0.0048659444, 0.43041354],
//     [-0.21918549, -0.44939706, 0.0], [0.28081453, 0.949397],
//     [0.35966998, -0.3473291, 0.0], [0.85967, 0.84732914],
//     [0.44147372, 0.2347359, 0.0], [0.9414737, 0.2652641]
// );
