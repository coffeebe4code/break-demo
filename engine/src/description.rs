use crate::{layout::Layout, texture::Texture};

pub type DescriptionId = u32;

pub struct Description<'t> {
    pub texture: &'t Texture,
    pub diffuse_bind_group: wgpu::BindGroup,
}

impl<'t> Description<'t> {
    pub fn new(
        texture: &'t Texture,
        device: &wgpu::Device,
        entries: Vec<wgpu::BindGroupEntry>,
        layout: &Layout,
        name: &str,
    ) -> Self {
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout.bind_group_layout,
            entries: &entries,
            label: Some(&format!("diffuse_bind_group: {}", name)),
        });
        Self {
            texture,
            diffuse_bind_group,
        }
    }
}
