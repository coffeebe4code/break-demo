use wgpu::BindGroupLayout;

use crate::{context::Context, layout::Layout};

pub struct Pipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(layout: &Layout, context: &Context, name: &str) -> Self {
        let groups: Vec<&BindGroupLayout> = layout.bind_group_layouts.iter().collect();
        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(&format!("pipeline_layout: {}", name)),
                    bind_group_layouts: &groups,
                    push_constant_ranges: &[],
                });
        let pipeline = context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(&format!("pipeline: {}", name)),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &layout.shader,
                    entry_point: Some("vs_main"),
                    buffers: &[layout.vertex_buffer_layout.to_owned()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &layout.shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: context.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });
        Self { pipeline }
    }
}
