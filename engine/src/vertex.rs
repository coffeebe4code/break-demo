use wgpu::{BindGroupLayoutEntry, VertexAttribute};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2DTexture {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
pub fn vertex(pos: [f32; 2], tc: [i8; 2]) -> Vertex2DTexture {
    Vertex2DTexture {
        position: [pos[0] as f32, pos[1] as f32],
        tex_coords: [tc[0] as f32, tc[1] as f32],
    }
}

const ATTRIBUTES: &'static [VertexAttribute] = &[
    wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Float32x2,
    },
    wgpu::VertexAttribute {
        offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
        shader_location: 1,
        format: wgpu::VertexFormat::Float32x2,
    },
];

//const BGLE: &'static [BindGroupLayoutEntry] = &[
//    wgpu::BindGroupLayoutEntry {
//        binding: 0,
//        visibility: wgpu::ShaderStages::FRAGMENT,
//        ty: wgpu::BindingType::Texture {
//            multisampled: false,
//            view_dimension: wgpu::TextureViewDimension::D2,
//            sample_type: wgpu::TextureSampleType::Float { filterable: true },
//        },
//        count: None,
//    },
//    wgpu::BindGroupLayoutEntry {
//        binding: 1,
//        visibility: wgpu::ShaderStages::FRAGMENT,
//        // This should match the filterable field of the
//        // corresponding Texture entry above.
//        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
//        count: None,
//    },
//];
//
//impl Vertex2DTexture {
//    pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
//        array_stride: size_of::<Self>() as wgpu::BufferAddress,
//        step_mode: wgpu::VertexStepMode::Vertex,
//        attributes: &ATTRIBUTES,
//    };
//}
