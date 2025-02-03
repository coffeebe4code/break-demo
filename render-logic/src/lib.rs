use engine::{context::Context, description::BindGroupType, scene::Scene, texture::Texture};
use wgpu::VertexAttribute;

pub struct IntroContainer<'a> {
    pub scene: Scene<'a>,
    pub attributes: Vec<VertexAttribute>,
}

impl<'a> IntroContainer<'a> {
    pub fn new(context: &'a Context) -> Self {
        let attributes = vec![
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
        let diffuse_bytes_pb = include_bytes!("../../assets/PowerBorder.png");
        let diffuse_bytes_b = include_bytes!("../../assets/B.png");
        let diffuse_bytes_r = include_bytes!("../../assets/R.png");
        let diffuse_bytes_e = include_bytes!("../../assets/E.png");
        let diffuse_bytes_a = include_bytes!("../../assets/A.png");
        let diffuse_bytes_k = include_bytes!("../../assets/K.png");
        let diffuse_bytes_x = include_bytes!("../../assets/I.png");

        let texture_pb = Texture::from_bytes(
            &context.device,
            &context.queue,
            diffuse_bytes_pb,
            "PowerBorder",
        )
        .unwrap();

        let texture_b =
            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_b, "B").unwrap();
        let texture_r =
            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_r, "R").unwrap();
        let texture_e =
            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_e, "E").unwrap();
        let texture_a =
            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_a, "A").unwrap();

        let texture_k =
            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_k, "K").unwrap();
        let texture_i =
            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_x, "I").unwrap();

        let mut scene = Scene::new(context)
            .add_attributes("basic", &attributes)
            .add_texture("PowerBorder", texture_pb)
            .add_texture("B", texture_b)
            .add_texture("R", texture_r)
            .add_texture("E", texture_e)
            .add_texture("A", texture_a)
            .add_texture("K", texture_k)
            .add_texture("I", texture_i);

        scene.add_layout(
            "standard layout",
            "basic",
            include_str!("../../assets/shader.wgsl"),
        );
        scene.add_description(
            "b entries",
            "B",
            &[BindGroupType::TextureView, BindGroupType::Sampler],
            "standard layout",
        );

        Self { scene, attributes }
    }
}
//    pub fn build_layout(mut self, context: &Context) -> Self {
//        let attr = self.attributes.unwrap();
//        let layout = Layout::new(
//            &self.attributes,
//            &context.device,
//            include_str!("../../assets/shader.wgsl"),
//            "intro",
//        );
//        self.layout = Some(layout);
//        return self;
//    }
//    pub fn textures(mut self, context: &Context) -> Self {
//        let diffuse_bytes_pb = include_bytes!("../../assets/PowerBorder.png");
//        let diffuse_bytes_b = include_bytes!("../../assets/B.png");
//        let diffuse_bytes_r = include_bytes!("../../assets/R.png");
//        let diffuse_bytes_e = include_bytes!("../../assets/E.png");
//        let diffuse_bytes_a = include_bytes!("../../assets/A.png");
//        let diffuse_bytes_k = include_bytes!("../../assets/K.png");
//        let diffuse_bytes_x = include_bytes!("../../assets/I.png");
//
//        let texture_pb = Texture::from_bytes(
//            &context.device,
//            &context.queue,
//            diffuse_bytes_pb,
//            "PowerBorder",
//        )
//        .unwrap();
//
//        let mut textures = vec![];
//
//        let texture_b =
//            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_b, "B").unwrap();
//        let texture_r =
//            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_r, "R").unwrap();
//        let texture_e =
//            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_e, "E").unwrap();
//        let texture_a =
//            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_a, "A").unwrap();
//
//        let texture_k =
//            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_k, "K").unwrap();
//        let texture_x =
//            Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_x, "I").unwrap();
//
//        textures.push(texture_b);
//        textures.push(texture_r);
//        textures.push(texture_e);
//        textures.push(texture_a);
//        textures.push(texture_k);
//        textures.push(texture_x);
//        self.textures = textures;
//        return self;
//    }
//    pub fn descriptions(mut self) -> Self {
//        let entry_b = vec![
//            wgpu::BindGroupEntry {
//                binding: 0,
//                resource: wgpu::BindingResource::TextureView(&textures[0].view),
//            },
//            wgpu::BindGroupEntry {
//                binding: 1,
//                resource: wgpu::BindingResource::Sampler(&textures[0].sampler),
//            },
//        ];
//        let entry_r = vec![
//            wgpu::BindGroupEntry {
//                binding: 0,
//                resource: wgpu::BindingResource::TextureView(&textures[1].view),
//            },
//            wgpu::BindGroupEntry {
//                binding: 1,
//                resource: wgpu::BindingResource::Sampler(&textures[1].sampler),
//            },
//        ];
//        let mut descriptions = vec![];
//
//        descriptions.push(Description::new(
//            &textures[0],
//            &context.device,
//            entry_b,
//            &layout,
//            "B",
//        ));
//
//        descriptions.push(Description::new(
//            &textures[1],
//            &context.device,
//            entry_r,
//            &layout,
//            "R",
//        ));
//    }
//
//    pub fn pipeline(mut self, context: &Context) -> Self {
//        let pipeline = Pipeline::new(
//            &self.layout.unwrap(),
//            &self.descriptions.unwrap(),
//            &context.device,
//            "intro",
//            &context.config,
//        );
//        self.pipeline = Some(pipeline);
//        self
//    }
//}
