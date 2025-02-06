use crate::layout::Layout;
use wgpu::BindGroupEntry;
pub type DescriptionId = u32;

#[derive(Clone, PartialEq, Eq)]
pub enum BindGroupType {
    Sampler,
    TextureView,
}

#[derive(Debug)]
pub struct Description {
    pub diffuse_bind_groups: Vec<wgpu::BindGroup>,
}

impl Description {
    pub fn new(
        entries: &[&[BindGroupEntry]],
        device: &wgpu::Device,
        layout: &Layout,
        name: &str,
    ) -> Self {
        let mut diffuse_bind_groups = vec![];
        for (i, x) in layout.bind_group_layouts.iter().enumerate() {
            let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &x,
                entries: &entries[i],
                label: Some(&format!("diffuse_bind_group: {} : {}", name, i)),
            });
            diffuse_bind_groups.push(diffuse_bind_group)
        }
        Self {
            diffuse_bind_groups,
        }
    }
}
