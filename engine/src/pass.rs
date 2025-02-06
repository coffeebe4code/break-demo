use std::rc::Rc;
use wgpu::RenderPass;

use crate::context::Context;
use crate::pipeline::Pipeline;
use crate::vertex::Vertex2DTexture;

pub struct PipelinePass {
    pub pipeline: Rc<Pipeline>,
    // todo:: could look into using the same vert_buffer since it uses indexes
    pub vert_buffers: Vec<wgpu::Buffer>,
    pub index_buffers: Vec<wgpu::Buffer>,
}

impl<'a> PipelinePass {
    pub fn new(pipeline: Rc<Pipeline>) -> Self {
        Self {
            pipeline,
            vert_buffers: vec![],
            index_buffers: vec![],
        }
    }
    pub fn configure_description_buffers(&mut self, queue: &wgpu::Queue, device: &wgpu::Device) {
        for (i, x) in self.pipeline.descriptions.iter().enumerate() {
            let vert_buff = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("vert buff description id: {}", i)),
                size: (1 * std::mem::size_of::<Vertex2DTexture>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let index_buff = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("index buff description id: {}", i)),
                size: (1 * std::mem::size_of::<u16>()) as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            self.vert_buffers.push(vert_buff);
            self.index_buffers.push(index_buff);

            queue.write_buffer(
                &self.vert_buffers.get(i as usize).unwrap(),
                0,
                bytemuck::cast_slice(&Vec::<Vertex2DTexture>::with_capacity(1)),
            );

            queue.write_buffer(
                &self.index_buffers.get(i as usize).unwrap(),
                0,
                bytemuck::cast_slice(&Vec::<u16>::with_capacity(1)),
            );
        }
    }
    pub fn update(
        &mut self,
        context: &Context,
        description_id: u32,
        index_arr: &[u16],
        vertex_arr: &[Vertex2DTexture],
    ) {
        let index = description_id as usize;
        let vertex_size = (vertex_arr.len() * std::mem::size_of::<Vertex2DTexture>()) as u64;
        let index_size = (index_arr.len() * std::mem::size_of::<u16>()) as u64;
        if self.vert_buffers.get(index).unwrap().size() >= vertex_size {
        } else {
            let vert_buff = context.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("vert buff description id: {}", description_id)),
                size: vertex_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let index_buff = context.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("index buff description id: {}", description_id)),
                size: index_size,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.vert_buffers[index] = vert_buff;
            self.index_buffers[index] = index_buff;
        }
        context.queue.write_buffer(
            &self.vert_buffers.get(index as usize).unwrap(),
            0,
            bytemuck::cast_slice(vertex_arr),
        );

        context.queue.write_buffer(
            &self.index_buffers.get(index as usize).unwrap(),
            0,
            bytemuck::cast_slice(index_arr),
        );
    }
    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.pipeline.pipeline);
        for (idx, id) in self.pipeline.descriptions.iter().enumerate() {
            render_pass.set_vertex_buffer(0, self.vert_buffers.get(idx).unwrap().slice(..));
            render_pass.set_index_buffer(
                self.index_buffers.get(idx).unwrap().slice(..),
                wgpu::IndexFormat::Uint16,
            );
            for (bgid, bg) in self.pipeline.descriptions[idx]
                .diffuse_bind_groups
                .iter()
                .enumerate()
            {
                render_pass.set_bind_group(bgid as u32, bg, &[]);
            }
            let index_len = (self.index_buffers[idx].size() >> 1) as u32;
            render_pass.draw_indexed(0..index_len, 0, 0..1);
        }
    }
}
