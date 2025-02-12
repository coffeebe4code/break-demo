use glyphon::*;
use wgpu::{MultisampleState, TextureFormat};

use crate::{context::Context, description::Descriptions};

pub struct Font {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub viewport: glyphon::Viewport,
    pub atlas: glyphon::TextAtlas,
    pub text_renderer: glyphon::TextRenderer,
    pub text_buffer: glyphon::Buffer,
}

pub struct FontColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FontColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Font {
    pub fn new(context: &Context, text: &str) -> Self {
        let mut font_system = FontSystem::new();
        let font_data = include_bytes!("../../assets/basic_sans_serif_7.ttf");

        // Register the font
        font_system.db_mut().load_font_data(font_data.to_vec());
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
            text,
            Attrs::new().family(Family::Name("Basic Sans Serif 7")),
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
                        left: 1000,
                        top: 1000,
                        right: 1000,
                        bottom: 1000,
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
    pub fn update(&mut self, context: &Context, fc: FontColor, location: (f32, f32)) {
        self.viewport.update(
            &context.queue,
            Resolution {
                width: context.config.width,
                height: context.config.height,
            },
        );

        self.text_renderer
            .prepare(
                &context.device,
                &context.queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                [TextArea {
                    buffer: &self.text_buffer,
                    left: location.0,
                    top: location.1,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: context.config.width as i32,
                        bottom: context.config.height as i32,
                    },
                    default_color: Color::rgb(fc.r, fc.g, fc.b),
                    custom_glyphs: &[],
                }],
                &mut self.swash_cache,
            )
            .unwrap();
    }
}

impl Descriptions for Font {
    fn render(&self, render_pass: &mut wgpu::RenderPass) -> () {
        self.text_renderer
            .render(&self.atlas, &self.viewport, render_pass)
            .unwrap();
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
