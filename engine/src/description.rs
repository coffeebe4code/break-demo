use std::any::Any;

use crate::{context::Context, layout::Layout};
use wgpu::{BindGroupEntry, RenderPass};

pub trait Descriptions {
    fn new(entries: &[&[BindGroupEntry]], context: &Context, layout: &Layout, name: &str) -> Self
    where
        Self: Sized;
    fn render(&self, render_pass: &mut RenderPass) -> ();
    fn as_any(&mut self) -> &mut dyn Any;
}

pub fn update_desc(
    desc: &mut dyn Descriptions,
    context: &Context,
    func: &mut dyn FnMut(&Context, &mut dyn Descriptions) -> (),
) {
    func(context, desc);
}
pub struct TextureDescription {
    pub diffuse_bind_groups: Vec<wgpu::BindGroup>,
    pub index_buffer: Option<wgpu::Buffer>,
}

impl Descriptions for TextureDescription {
    fn new(entries: &[&[BindGroupEntry]], context: &Context, layout: &Layout, name: &str) -> Self {
        let mut diffuse_bind_groups = vec![];
        for (i, x) in layout.bind_group_layouts.iter().enumerate() {
            let diffuse_bind_group = context
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &x,
                    entries: &entries[i],
                    label: Some(&format!("diffuse_bind_group: {} : {}", name, i)),
                });
            diffuse_bind_groups.push(diffuse_bind_group)
        }
        Self {
            index_buffer: None,
            diffuse_bind_groups,
        }
    }
    fn render(&self, render_pass: &mut RenderPass) -> () {
        let ib = self.index_buffer.as_ref().unwrap();
        render_pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint16);

        for (bgid, bg) in self.diffuse_bind_groups.iter().enumerate() {
            render_pass.set_bind_group(bgid as u32, bg, &[]);
        }
        let index_len = (ib.size() >> 1) as u32;
        render_pass.draw_indexed(0..index_len, 0, 0..1);
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
