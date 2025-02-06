use std::borrow::Cow;

use wgpu::BindGroupLayout;
use wgpu::BindGroupLayoutEntry;
use wgpu::Device;
use wgpu::ShaderModule;
use wgpu::VertexAttribute;
use wgpu::VertexBufferLayout;

use crate::vertex::Vertex2D;
use crate::vertex::Vertex2DTexture;

#[derive(Debug)]
pub struct Layout {
    pub vertex_buffer_layout: VertexBufferLayout<'static>,
    pub bind_group_layouts: Vec<BindGroupLayout>,
    pub shader: ShaderModule,
}

pub trait LayoutSize {
    fn size() -> usize;
}

impl LayoutSize for Vertex2DTexture {
    fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

impl LayoutSize for Vertex2D {
    fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

impl Layout {
    pub fn new(
        attrs: &'static [VertexAttribute],
        bind_layouts: &[&'static [BindGroupLayoutEntry]],
        device: &Device,
        shader_source: &'static str,
        name: &str,
        size: usize,
    ) -> Self {
        let bind_group_layouts: Vec<BindGroupLayout> = bind_layouts
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let bind_group_layout =
                    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        entries: *x,
                        label: Some(&format!("bind_group_layout: {}: {}", name, i)),
                    });
                bind_group_layout
            })
            .collect();

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("shader: {}", name)),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_source)).into(),
        });

        Self {
            shader,
            bind_group_layouts,
            vertex_buffer_layout,
        }
    }
}
