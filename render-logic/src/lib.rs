use engine::{
    context::Context,
    layout::LayoutSize,
    scene::Scene,
    vertex::{Vertex2D, Vertex2DTexture},
};
use wgpu::{BindGroupLayoutEntry, VertexAttribute};

use engine::description::*;

pub struct Background {
    //pub index_buffer: Option<wgpu::Buffer>,
}

impl Descriptions for Background {
    fn new(
        _entries: &[&[wgpu::BindGroupEntry]],
        _context: &Context,
        _layout: &engine::layout::Layout,
        _name: &str,
    ) -> Self
    where
        Self: Sized,
    {
        return Self {};
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) -> () {
        render_pass.draw(0..6, 0..1);
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct IntroContainer {
    pub scene: Scene,
}

impl IntroContainer {
    pub fn new(context: &Context) -> Self {
        const BACKGROUND_ATTRIBUTES: &'static [VertexAttribute] = &[wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x2,
        }];
        const ATTRIBUTES: &'static [VertexAttribute] = &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x2,
            },
        ];

        const TEXTURE_LAYOUT_ENTRY: &'static [BindGroupLayoutEntry] = &[
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
        ];

        let scene = Scene::new()
            .add_texture("PowerBorder", "./assets/PowerBorder.png", context)
            .add_texture("B", "./assets/B.png", context)
            .add_texture("R", "./assets/R.png", context)
            .add_texture("E", "./assets/E.png", context)
            .add_texture("A", "./assets/A.png", context)
            .add_texture("K", "./assets/K.png", context)
            .add_texture("I", "./assets/I.png", context)
            .add_layout(
                "standard layout",
                ATTRIBUTES,
                &[TEXTURE_LAYOUT_ENTRY],
                include_str!("../../assets/shader.wgsl"),
                context,
                Vertex2DTexture::size(),
            )
            .add_texture_description("b", "B", "standard layout", context)
            .add_texture_description("r", "R", "standard layout", context)
            .add_layout(
                "background layout",
                BACKGROUND_ATTRIBUTES,
                &[],
                include_str!("../../assets/background.wgsl"),
                context,
                Vertex2D::size(),
            )
            .add_font_desc(&context, "font", "background layout", &"hello,asdfasdfasdfasdfasdfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
            .add_description("background", Background {})
            .compile_pipeline(
                "background",
                &["background", "font"],
                "background layout",
                context,
            )
            .compile_pipeline("intro", &["b", "r"], "standard layout", context);
        Self { scene }
    }
}
