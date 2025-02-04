use wgpu::BindGroupEntry;

use crate::{layout::Layout, texture::Texture};

pub type DescriptionId = u32;

#[derive(Clone, PartialEq, Eq)]
pub enum BindGroupType {
    Sampler,
    TextureView,
}

#[derive(Debug)]
pub struct Description {
    pub diffuse_bind_group: wgpu::BindGroup,
}

impl Description {
    pub fn new(
        texture: &Texture,
        entries: &[BindGroupType],
        device: &wgpu::Device,
        layout: &Layout,
        name: &str,
    ) -> Self {
        let bgs: Vec<BindGroupEntry> = entries
            .iter()
            .enumerate()
            .map(|(i, x)| match x {
                BindGroupType::TextureView => {
                    return wgpu::BindGroupEntry {
                        binding: i as u32,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    }
                }
                BindGroupType::Sampler => {
                    return wgpu::BindGroupEntry {
                        binding: i as u32,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    }
                }
            })
            .collect();

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout.bind_group_layout,
            entries: &bgs,
            label: Some(&format!("diffuse_bind_group: {}", name)),
        });
        Self { diffuse_bind_group }
    }
}
