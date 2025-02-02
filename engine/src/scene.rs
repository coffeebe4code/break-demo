use std::collections::HashMap;

use wgpu::VertexAttribute;

use crate::context::Context;
use crate::description::Description;
use crate::layout::Layout;
use crate::pass::PipelinePass;
use crate::pipeline::Pipeline;
use crate::texture::Texture;

pub struct Scene<'a> {
    pub textures: HashMap<String, Texture>,
    pub descriptions: HashMap<String, Description<'a>>,
    pub attributes: HashMap<String, Vec<VertexAttribute>>,
    pub passes: HashMap<String, PipelinePass<'a>>,
    pub pipelines: HashMap<String, Pipeline<'a>>,
    pub layouts: HashMap<String, Layout<'a>>,
    pub pipeline_descriptions: HashMap<String, Vec<(String, u32)>>,
}

pub struct CompiledScene<'a> {
    pub textures: HashMap<String, Texture>,
    pub descriptions: HashMap<String, Description<'a>>,
    pub attributes: HashMap<String, Vec<VertexAttribute>>,
    pub pipelines: HashMap<String, PipelinePass<'a>>,
    pub layouts: HashMap<String, Layout<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            descriptions: HashMap::new(),
            attributes: HashMap::new(),
            passes: HashMap::new(),
            layouts: HashMap::new(),
            pipelines: HashMap::new(),
            pipeline_descriptions: HashMap::new(),
        }
    }
    pub fn add_attributes(mut self, name: &str, attrs: &[VertexAttribute]) -> Self {
        self.attributes.insert(name.to_string(), attrs.to_vec());
        self
    }
    pub fn add_texture(mut self, name: &str, texture: Texture) -> Self {
        self.textures.insert(name.to_string(), texture);
        self
    }
    pub fn add_layout(
        &'a mut self,
        name: &str,
        device: &wgpu::Device,
        attribute_name: &str,
        shader_source: &'static str,
    ) {
        let layout = Layout::new(
            &self.attributes[attribute_name],
            device,
            shader_source,
            name,
        );
        self.layouts.insert(name.to_string(), layout);
    }
    pub fn add_description(
        &'a mut self,
        name: &str,
        device: &wgpu::Device,
        texture_name: &str,
        entries: &[wgpu::BindGroupEntry],
        layout_name: &str,
    ) {
        let description = Description::new(
            &self.textures[texture_name],
            device,
            entries.to_vec(),
            &self.layouts[layout_name],
            name,
        );
        self.descriptions.insert(name.to_string(), description);
    }
    pub fn compile_pipeline(
        &'a mut self,
        name: &str,
        context: &Context,
        description_names: &[&str],
        layout_name: &str,
    ) {
        let mut des = vec![];
        for x in description_names {
            des.push(&self.descriptions[*x]);
        }
        let pipeline = Pipeline::new(
            &self.layouts[layout_name],
            des,
            &context.device,
            name,
            &context.config,
        );
        self.pipelines.insert(name.to_string(), pipeline);

        let temp = &self.pipelines[name];
        let mut pass = PipelinePass::new(temp);
        for (i, x) in temp.descriptions.iter().enumerate() {
            let id = pass.configure_description_buffers(
                i as u32,
                &context.queue,
                &context.device,
                // sus
                &vec![],
                &vec![],
            );
        }
        self.passes.insert(name.to_string(), pass);
    }
}
