use std::cell::RefCell;
use std::rc::Rc;
use wgpu::RenderPass;

use crate::context::Context;
use crate::description::update_desc;
use crate::description::Descriptions;
use crate::pipeline::Pipeline;

pub struct PipelinePass {
    pub pipeline: Rc<Pipeline>,
    pub descriptions: Vec<Rc<RefCell<Box<dyn Descriptions>>>>,
    pub vert_buffer: Option<wgpu::Buffer>,
}

impl<'a> PipelinePass {
    pub fn new(pipeline: Rc<Pipeline>) -> Self {
        Self {
            pipeline,
            descriptions: vec![],
            vert_buffer: None,
        }
    }
    pub fn add_desc(&mut self, desc: Vec<Rc<RefCell<Box<dyn Descriptions>>>>) {
        self.descriptions.extend(desc);
    }
    pub fn update(
        &mut self,
        context: &Context,
        description_id: u32,
        func: &mut dyn FnMut(&Context, &mut dyn Descriptions) -> (),
    ) {
        let temp: &mut Rc<RefCell<Box<dyn Descriptions>>> =
            self.descriptions.get_mut(description_id as usize).unwrap();
        let desc = &mut **temp.borrow_mut();

        update_desc(desc, context, func);
    }
    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.pipeline.pipeline);
        for desc in self.descriptions.iter() {
            render_pass.set_vertex_buffer(0, self.vert_buffer.as_ref().unwrap().slice(..));
            desc.borrow_mut().render(render_pass);
        }
    }
}
