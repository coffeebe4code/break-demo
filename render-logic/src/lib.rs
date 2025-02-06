use engine::{context::Context, layout::LayoutSize, scene::Scene, vertex::Vertex2DTexture};
use wgpu::{BindGroupLayoutEntry, SurfaceError, VertexAttribute};

pub struct IntroContainer {
    pub scene: Scene,
}

impl IntroContainer {
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
            .add_texture_description("b description", "B", "standard layout", context)
            .add_texture_description("r description", "R", "standard layout", context)
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
