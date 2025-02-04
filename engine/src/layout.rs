use std::borrow::Cow;

use wgpu::BindGroupLayout;
use wgpu::BindGroupLayoutEntry;
use wgpu::Device;
use wgpu::PipelineLayout;
use wgpu::ShaderModule;
use wgpu::VertexAttribute;
use wgpu::VertexBufferLayout;

use crate::vertex::Vertex2DTexture;

#[derive(Clone, PartialEq, Eq)]
pub enum BindLayoutType {
    SamplerFragment,
    TextureFragment,
}

#[derive(Debug)]
pub struct Layout<'a> {
    pub pipeline_layout: PipelineLayout,
    pub vertex_buffer_layout: VertexBufferLayout<'a>,
    pub bind_group_layout: BindGroupLayout,
    pub shader: ShaderModule,
}

impl<'a> Layout<'a> {
    pub fn new(
        attrs: &'a [VertexAttribute],
        bind_layouts: &'a [BindLayoutType],
        device: &Device,
        shader_source: &'static str,
        name: &str,
    ) -> Self {
        let bls: Vec<BindGroupLayoutEntry> = bind_layouts
            .iter()
            .enumerate()
            .map(|(i, x)| match x {
                BindLayoutType::TextureFragment => {
                    return wgpu::BindGroupLayoutEntry {
                        binding: i as u32,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    };
                }
                BindLayoutType::SamplerFragment => {
                    return wgpu::BindGroupLayoutEntry {
                        binding: i as u32,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    };
                }
            })
            .collect();
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &bls,
            label: Some(&format!("bind_group_layout: {}", name)),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("pipeline_layout: {}", name)),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex2DTexture>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &attrs,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("shader: {}", name)),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_source)).into(),
        });

        Self {
            pipeline_layout,
            shader,
            vertex_buffer_layout,
            bind_group_layout,
        }
    }
}
