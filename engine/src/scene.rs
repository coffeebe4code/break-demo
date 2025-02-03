use std::collections::HashMap;
use std::rc::Rc;

use wgpu::VertexAttribute;

use crate::context::Context;
use crate::description::{BindGroupType, Description};
use crate::layout::Layout;
use crate::pass::PipelinePass;
use crate::pipeline::Pipeline;
use crate::texture::Texture;
use crate::vertex::Vertex2DTexture;

pub struct Scene<'a> {
    pub textures: HashMap<String, Texture>,
    pub descriptions: HashMap<String, Rc<Description>>,
    pub passes: HashMap<String, PipelinePass>,
    pub pipelines: HashMap<String, Rc<Pipeline>>,
    pub layouts: HashMap<String, Layout<'a>>,
    pub lookups: HashMap<String, Vec<String>>,
}

impl<'a> Scene<'a> {
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
        attrs: &'a [VertexAttribute],
        shader_source: &'static str,
        context: &Context,
    ) -> Self {
        let layout = Layout::new(attrs, &context.device, shader_source, name);
        println!("layout {:#?}", layout);
        self.layouts.insert(name.to_string(), layout);
        self
    }
    pub fn add_description(
        mut self,
        name: &str,
        texture_name: &str,
        entries: &[BindGroupType],
        layout_name: &str,
        context: &Context,
    ) -> Self {
        let description = Description::new(
            &self.textures[texture_name],
            entries,
            &context.device,
            &self.layouts[layout_name],
            name,
        );
        println!("description {:#?}", description);
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
}
