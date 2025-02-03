use wgpu::BindGroupEntry;

use crate::{layout::Layout, texture::Texture};

pub type DescriptionId = u32;

#[derive(Clone)]
pub enum BindGroupType {
    Sampler,
    TextureView,
}

pub struct Description<'t> {
    pub entries: Vec<BindGroupType>,
    pub texture: Option<&'t Texture>,
    pub diffuse_bind_group: Option<wgpu::BindGroup>,
}

impl<'t> Description<'t> {
    pub fn update(
        &mut self,
        texture: &'t Texture,
        device: &wgpu::Device,
        layout: &Layout,
        name: &str,
    ) {
        let bgs: Vec<BindGroupEntry> = self
            .entries
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
    }
    pub fn new(entries: Vec<BindGroupType>) -> Self {
        Self {
            entries,
            texture: None,
            diffuse_bind_group: None,
        }
    }
}
