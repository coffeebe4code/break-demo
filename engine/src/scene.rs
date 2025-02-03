use std::collections::HashMap;

use wgpu::VertexAttribute;

use crate::context::Context;
use crate::description::{BindGroupType, Description};
use crate::layout::Layout;
use crate::pass::PipelinePass;
use crate::pipeline::Pipeline;
use crate::texture::Texture;

pub struct Scene<'a> {
    pub context: &'a Context,
    pub textures: HashMap<String, Texture>,
    pub descriptions: HashMap<String, Description<'a>>,
    pub attributes: HashMap<String, Vec<VertexAttribute>>,
    pub passes: HashMap<String, PipelinePass<'a>>,
    pub pipelines: HashMap<String, Pipeline<'a>>,
    pub layouts: HashMap<String, Layout<'a>>,
    pub pipeline_descriptions_lookup: HashMap<String, Vec<(String, u32)>>,
}

impl<'a, 'b: 'a> Scene<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            textures: HashMap::new(),
            descriptions: HashMap::new(),
            attributes: HashMap::new(),
            passes: HashMap::new(),
            layouts: HashMap::new(),
            pipelines: HashMap::new(),
            pipeline_descriptions_lookup: HashMap::new(),
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
    pub fn add_layout(&'b mut self, name: &str, attribute_name: &str, shader_source: &'static str) {
        let layout = Layout::new(
            &self.attributes[attribute_name],
            &self.context.device,
            shader_source,
            name,
        );
        self.layouts.insert(name.to_string(), layout);
    }
    pub fn add_description(
        &'b mut self,
        name: &str,
        texture_name: &str,
        entries: &[BindGroupType],
        layout_name: &str,
    ) {
        let mut description = Description::new(entries.to_vec());
        description.update(
            &self.textures[texture_name],
            &self.context.device,
            &self.layouts[layout_name],
            name,
        );
        self.descriptions.insert(name.to_string(), description);
    }
    pub fn compile_pipeline(
        &'a mut self,
        name: &str,
        description_names: &[&str],
        layout_name: &str,
    ) {
        let mut des = vec![];
        for x in description_names {
            des.push(&self.descriptions[*x]);
        }
        let pipeline = Pipeline::new(&self.layouts[layout_name], des, &self.context, name);
        self.pipelines.insert(name.to_string(), pipeline);

        let temp = &self.pipelines[name];
        let mut pass = PipelinePass::new(temp);
        for (i, x) in temp.descriptions.iter().enumerate() {
            let id = pass.configure_description_buffers(
                i as u32,
                &self.context.queue,
                &self.context.device,
                // sus
                &vec![],
                &vec![],
            );
        }
        self.passes.insert(name.to_string(), pass);
    }
}
