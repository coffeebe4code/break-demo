use glyphon::*;
use wgpu::{MultisampleState, TextureFormat};

use crate::{context::Context, description::Descriptions, layout::Layout};

pub struct Font {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub viewport: glyphon::Viewport,
    pub atlas: glyphon::TextAtlas,
    pub text_renderer: glyphon::TextRenderer,
    pub text_buffer: glyphon::Buffer,
}

impl Descriptions for Font {
    fn new(
        _entries: &[&[wgpu::BindGroupEntry]],
        context: &Context,
        _layout: &Layout,
        name: &str,
    ) -> Self {
        let mut font_system = FontSystem::new();
        let mut swash_cache = SwashCache::new();
        let cache = Cache::new(&context.device);
        let mut viewport = Viewport::new(&context.device, &cache);
        let mut atlas = TextAtlas::new(
            &context.device,
            &context.queue,
            &cache,
            TextureFormat::Bgra8UnormSrgb,
        );
        let mut text_renderer = TextRenderer::new(
            &mut atlas,
            &context.device,
            MultisampleState::default(),
            None,
        );
        let mut text_buffer = Buffer::new(&mut font_system, Metrics::new(30.0, 42.0));

        let physical_width = (context.config.width as f64) as f32;
        let physical_height = (context.config.height as f64) as f32;

        text_buffer.set_size(
            &mut font_system,
            Some(physical_width),
            Some(physical_height),
        );
        text_buffer.set_text(
            &mut font_system,
            name,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );

        text_buffer.shape_until_scroll(&mut font_system, false);
        viewport.update(
            &context.queue,
            Resolution {
                width: context.config.width,
                height: context.config.height,
            },
        );
        text_renderer
            .prepare(
                &context.device,
                &context.queue,
                &mut font_system,
                &mut atlas,
                &viewport,
                [TextArea {
                    buffer: &text_buffer,
                    left: 10.0,
                    top: 10.0,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 50,
                        top: (context.config.height / 2) as i32 + 50,
                        right: context.config.width as i32 - 100,
                        bottom: context.config.width as i32 - 100,
                    },
                    default_color: Color::rgb(255, 255, 255),
                    custom_glyphs: &[],
                }],
                &mut swash_cache,
            )
            .unwrap();

        Self {
            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,
        }
    }

    fn render(&self, render_pass: &mut wgpu::RenderPass) -> () {
        self.text_renderer
            .render(&self.atlas, &self.viewport, render_pass)
            .unwrap();
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
