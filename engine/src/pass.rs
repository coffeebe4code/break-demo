use crate::texture::Texture;
use crate::vertex::Vertex;
use wgpu::util::DeviceExt;

pub struct Pass<'b> {
    pub textures: Vec<Texture>,
    pub diffuse_bind_groups: Vec<wgpu::BindGroup>,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub render_pipeline: wgpu::RenderPipeline,
    pub indexes: Vec<&'b [u16]>,
    pub vertexes: Vec<&'b [Vertex]>,
    pub vert_buffers: Vec<wgpu::Buffer>,
    pub index_buffers: Vec<wgpu::Buffer>,
}

impl<'b> Pass<'b> {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        vertex_attributes: &[wgpu::VertexAttribute],
    ) -> Self {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("WG Pass Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../assets/shader.wgsl").into()),
        });

        let vlayout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &vertex_attributes,
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vlayout],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });
        Self {
            textures: vec![],
            texture_bind_group_layout,
            diffuse_bind_groups: vec![],
            render_pipeline: pipeline,
            indexes: vec![],
            vertexes: vec![],
            vert_buffers: vec![],
            index_buffers: vec![],
        }
    }
    pub fn add(
        &mut self,
        index: &'b [u16],
        vertex: &'b [Vertex],
        texture: Texture,
        device: &wgpu::Device,
    ) {
        self.vertexes.push(vertex);
        self.indexes.push(index);
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });
        self.diffuse_bind_groups.push(diffuse_bind_group);
        self.textures.push(texture);
    }
    pub fn build(&mut self, device: &wgpu::Device) {
        for (i, _) in self.textures.iter().enumerate() {
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Flat Vertex Buffer"),
                contents: bytemuck::cast_slice(self.vertexes.get(i).unwrap()),
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.vert_buffers.push(vertex_buffer);
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Flat Index Buffer"),
                contents: bytemuck::cast_slice(self.indexes.get(i).unwrap()),
                usage: wgpu::BufferUsages::INDEX,
            });
            self.index_buffers.push(index_buffer);
        }
    }
}

impl<'b> Passable for Pass<'b> {
    fn render(
        &mut self,
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("WG Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            for (x, _) in self.vertexes.iter().enumerate() {
                render_pass.set_vertex_buffer(0, self.vert_buffers.get(x).unwrap().slice(..));
                render_pass.set_index_buffer(
                    self.index_buffers.get(x).unwrap().slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.set_bind_group(0, self.diffuse_bind_groups.get(x).unwrap(), &[]);

                let index = *self.indexes.get(x).unwrap();
                render_pass.draw_indexed(0..index.len() as u32, 0, 0..1);
            }
        }

        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub trait Passable {
    fn render(
        &mut self,
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<(), wgpu::SurfaceError>;
}
