use engine::{
    context::Context, description::BindGroupType, layout::BindLayoutType, scene::Scene,
    vertex::Vertex2DTexture,
};
use wgpu::{SurfaceError, VertexAttribute};

pub struct IntroContainer<'a> {
    pub scene: Scene<'a>,
}

impl<'a> IntroContainer<'a> {
    pub fn new(context: &Context) -> Self {
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
        const NORMAL_TEXTURE: &'static [BindLayoutType] = &[
            BindLayoutType::TextureFragment,
            BindLayoutType::SamplerFragment,
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
                NORMAL_TEXTURE,
                include_str!("../../assets/shader.wgsl"),
                context,
            )
            .add_description(
                "b description",
                "B",
                &[BindGroupType::TextureView, BindGroupType::Sampler],
                "standard layout",
                context,
            )
            .add_description(
                "r description",
                "R",
                &[BindGroupType::TextureView, BindGroupType::Sampler],
                "standard layout",
                context,
            )
            .compile_pipeline(
                "intro",
                &["b description", "r description"],
                "standard layout",
                context,
            );
        Self { scene }
    }
    pub fn update(
        &mut self,
        pipeline_name: &str,
        description: &str,
        vbuf: &[Vertex2DTexture],
        ibuf: &[u16],
        context: &Context,
    ) {
        self.scene
            .update(pipeline_name, description, ibuf, vbuf, context);
    }
    pub fn render(&self, pipeline_names: &[&str], context: &Context) -> Result<(), SurfaceError> {
        self.scene.render(pipeline_names, context)
    }
}
