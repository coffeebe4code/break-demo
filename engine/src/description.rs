use crate::{layout::Layout, texture::Texture};

pub struct Description {
    pub texture: Texture,
    pub diffuse_bind_group: wgpu::BindGroup,
}

impl Description {
    pub fn new(
        texture: Texture,
        device: &wgpu::Device,
        entries: &[wgpu::BindGroupEntry],
        layout: &Layout,
        name: &'static str,
    ) -> Self {
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout.bind_group_layout,
            entries,
            label: Some(&format!("diffuse_bind_group: {}", name)),
        });
        Self {
            texture,
            diffuse_bind_group,
        }
    }
}
