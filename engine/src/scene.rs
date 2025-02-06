use std::collections::HashMap;
use std::rc::Rc;

use wgpu::{BindGroupEntry, BindGroupLayoutEntry, VertexAttribute};

use crate::context::Context;
use crate::description::{BindGroupType, Description};
use crate::layout::Layout;
use crate::pass::PipelinePass;
use crate::pipeline::Pipeline;
use crate::texture::Texture;
use crate::vertex::Vertex2DTexture;

pub struct Scene {
    pub textures: HashMap<String, Texture>,
    pub descriptions: HashMap<String, Rc<Description>>,
    pub passes: HashMap<String, PipelinePass>,
    pub pipelines: HashMap<String, Rc<Pipeline>>,
    pub layouts: HashMap<String, Layout>,
    pub lookups: HashMap<String, Vec<String>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            descriptions: HashMap::new(),
            passes: HashMap::new(),
            layouts: HashMap::new(),
            pipelines: HashMap::new(),
            lookups: HashMap::new(),
        }
    }
    pub fn add_texture(mut self, name: &str, file: &str, context: &Context) -> Self {
        let texture = Texture::from_file(&context.device, &context.queue, file, name).unwrap();
        self.textures.insert(name.to_string(), texture);
        self
    }
    pub fn add_layout(
        mut self,
        name: &str,
        attrs: &'static [VertexAttribute],
        bind_layouts: &[&'static [BindGroupLayoutEntry]],
        shader_source: &'static str,
        context: &Context,
        size: usize,
    ) -> Self {
        let layout = Layout::new(
            attrs,
            bind_layouts,
            &context.device,
            shader_source,
            name,
            size,
        );
        self.layouts.insert(name.to_string(), layout);
        self
    }
    pub fn add_texture_description(
        mut self,
        name: &str,
        texture_name: &str,
        layout_name: &str,
        context: &Context,
    ) -> Self {
        let temp = &self.textures[texture_name];
        let bges = &[
            wgpu::BindGroupEntry {
                binding: 0 as u32,
                resource: wgpu::BindingResource::TextureView(&temp.view),
            },
            wgpu::BindGroupEntry {
                binding: 1 as u32,
                resource: wgpu::BindingResource::Sampler(&temp.sampler),
            },
        ];

        let description =
            Description::new(&[bges], &context.device, &self.layouts[layout_name], name);
        self.descriptions
            .insert(name.to_string(), Rc::new(description));
        self
    }
    pub fn compile_pipeline(
        mut self,
        name: &str,
        description_names: &[&str],
        layout_name: &str,
        context: &Context,
    ) -> Self {
        let mut des = vec![];
        for x in description_names {
            des.push(self.descriptions[*x].clone());
        }
        let pipeline = Pipeline::new(&self.layouts[layout_name], des, &context, name);
        self.pipelines.insert(name.to_string(), Rc::new(pipeline));

        let mut pass = PipelinePass::new(self.pipelines[name].clone());
        pass.configure_description_buffers(&context.queue, &context.device);
        self.passes.insert(name.to_string(), pass);
        let vec: Vec<String> = description_names.iter().map(|x| x.to_string()).collect();
        self.lookups.insert(name.to_string(), vec);
        self
    }
    pub fn update(
        &mut self,
        pass_name: &str,
        description_name: &str,
        index_arr: &[u16],
        vertex_arr: &[Vertex2DTexture],
        context: &Context,
    ) {
        let mut idx = 0;
        for (i, x) in self.lookups[pass_name].iter().enumerate() {
            if x == description_name {
                idx = i
            }
        }
        self.passes
            .get_mut(pass_name)
            .unwrap()
            .update(&context, idx as u32, index_arr, vertex_arr);
    }
    pub fn render(
        &self,
        pipeline_names: &[&str],
        context: &Context,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = context.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
            for &x in pipeline_names {
                self.passes[x].render(&mut render_pass);
            }
        }

        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
