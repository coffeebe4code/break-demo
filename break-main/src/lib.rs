use engine::{
    buffers::index_buffer,
    context::Context,
    description::TextureDescription,
    font::Font,
    vertex::{vertex2d, vertex2dtexture},
    window::{Window, WindowEvents},
};
use glyphon::{Attrs, Family, Resolution, Shaping};
use render_logic::IntroContainer;
use winit::event_loop::EventLoop;

pub async fn main_work() -> () {
    let event_loop = EventLoop::new().unwrap();
    let window = Window::new(&event_loop);
    let mut context = Context::new(&window).await;
    let mut intro = IntroContainer::new(&context);

    let verts = vec![
        vertex2d([-1.0, 1.0]),
        vertex2d([1.0, 1.0]),
        vertex2d([1.0, -1.0]),
        vertex2d([1.0, -1.0]),
        vertex2d([-1.0, -1.0]),
        vertex2d([-1.0, 1.0]),
    ];
    let indices_b = &[0, 1, 2, 2, 3, 0];
    let b = vec![
        vertex2dtexture([0.0, 0.0], [0, 1]),
        vertex2dtexture([0.15, 0.0], [1, 1]),
        vertex2dtexture([0.15, 0.15], [1, 0]),
        vertex2dtexture([0.0, 0.15], [0, 0]),
        vertex2dtexture([0.15, 0.0], [0, 1]),
        vertex2dtexture([0.30, 0.0], [1, 1]),
        vertex2dtexture([0.30, 0.15], [1, 0]),
        vertex2dtexture([0.15, 0.15], [0, 0]),
    ];
    let indices_r = &[4, 5, 6, 6, 7, 4];
    intro.scene.update_verticies("intro", &context, &b);
    intro.scene.update_verticies("background", &context, &verts);

    intro.scene.update("intro", "b", &context, &mut |c, d| {
        let desc: &mut TextureDescription =
            d.as_any().downcast_mut::<TextureDescription>().unwrap();

        if let None = &desc.index_buffer {
            let index_buffer = index_buffer(&c, "b", indices_b);
            desc.index_buffer = Some(index_buffer);
            return;
        }
    });
    intro.scene.update("intro", "r", &context, &mut |c, d| {
        let desc: &mut TextureDescription =
            d.as_any().downcast_mut::<TextureDescription>().unwrap();

        if let None = &desc.index_buffer {
            let index_buffer = index_buffer(&c, "r", indices_r);
            desc.index_buffer = Some(index_buffer);
            return;
        }
    });
    window.run(event_loop, |event| match event {
        WindowEvents::Resized { width, height } => {
            context.resize(width, height);
            intro
                .scene
                .update("background", "font", &context, &mut |c, d| {
                    let desc: &mut Font = d.as_any().downcast_mut::<Font>().unwrap();

                    let physical_width = (c.config.width as f64) as f32;
                    let physical_height = (c.config.height as f64) as f32;

                    desc.text_buffer.set_size(
                        &mut desc.font_system,
                        Some(physical_width),
                        Some(physical_height),
                    );

                    desc.text_buffer.set_text(
                        &mut desc.font_system,
                        "HELLO THERE, THIS IS THE UPDATED ONE",
                        Attrs::new().family(Family::SansSerif),
                        Shaping::Advanced,
                    );
                    desc.viewport.update(
                        &context.queue,
                        Resolution {
                            width: context.config.width,
                            height: context.config.height,
                        },
                    );
                });
            window.request_redraw();
        }
        WindowEvents::Draw => {
            intro
                .scene
                .render(&["background", "intro"], &context)
                .unwrap();
            window.request_redraw();
        }
        _ => {}
    });
}
