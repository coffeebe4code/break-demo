use std::collections::HashMap;

use wgpu::VertexAttribute;

use crate::description::Description;
use crate::layout::Layout;
use crate::pass::PipelinePass;
use crate::texture::Texture;

pub struct Scene<'a> {
    pub textures: HashMap<String, Texture>,
    pub descriptions: HashMap<String, Description<'a>>,
    pub attributes: HashMap<String, Vec<VertexAttribute>>,
    pub pipelines: HashMap<String, PipelinePass<'a>>,
    pub layouts: HashMap<String, Layout<'a>>,
}

pub struct CompiledScene<'a> {
    pub textures: HashMap<String, Texture>,
    pub descriptions: HashMap<String, Description<'a>>,
    pub attributes: HashMap<String, Vec<VertexAttribute>>,
    pub pipeline: HashMap<String, PipelinePass<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            descriptions: HashMap::new(),
            attributes: HashMap::new(),
            pipelines: HashMap::new(),
            layouts: HashMap::new(),
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
    pub fn create_pipeline(&mut self, 
}
