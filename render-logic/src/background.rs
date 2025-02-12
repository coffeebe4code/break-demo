use wgpu::VertexAttribute;

pub struct Background {}

impl Background {
    pub const BACKGROUND_ATTRIBUTES: &'static [VertexAttribute] = &[wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Float32x2,
    }];
    pub fn new() -> Self {
        Self {}
    }
}
