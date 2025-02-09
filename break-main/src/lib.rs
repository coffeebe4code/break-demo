use engine::{
    buffers::index_buffer,
    context::Context,
    description::TextureDescription,
    vertex::vertex2dtexture,
    window::{Window, WindowEvents},
};
use render_logic::IntroContainer;
use winit::event_loop::EventLoop;

pub async fn main_work() -> () {
    let event_loop = EventLoop::new().unwrap();
    let window = Window::new(&event_loop);
    let mut context = Context::new(&window).await;
    let mut intro = IntroContainer::new(&context);

    let indices_b = &[0, 1, 2, 2, 3, 0];
    let b = vec![
        vertex2dtexture([0.0, 0.0], [0, 1]),
        vertex2dtexture([0.15, 0.0], [1, 1]),
        vertex2dtexture([0.15, 0.15], [1, 0]),
        vertex2dtexture([0.0, 0.15], [0, 0]),
        vertex2dtexture([0.15, 0.15], [0, 1]),
        vertex2dtexture([0.30, 0.15], [1, 1]),
        vertex2dtexture([0.30, 0.30], [1, 0]),
        vertex2dtexture([0.15, 0.30], [0, 0]),
    ];
    let indices_r = &[4, 5, 6, 6, 7, 4];
    intro.scene.update_verticies("intro", &context, &b);

    intro
        .scene
        .update("intro", "b description", &context, &mut |c, d| {
            let desc: &mut TextureDescription =
                d.as_any().downcast_mut::<TextureDescription>().unwrap();

            if let None = &desc.index_buffer {
                let index_buffer = index_buffer(&c, "b description", indices_b);
                desc.index_buffer = Some(index_buffer);
                return;
            }
        });
    intro
        .scene
        .update("intro", "r description", &context, &mut |c, d| {
            let desc: &mut TextureDescription =
                d.as_any().downcast_mut::<TextureDescription>().unwrap();

            if let None = &desc.index_buffer {
                let index_buffer = index_buffer(&c, "r description", indices_r);
                desc.index_buffer = Some(index_buffer);
                return;
            }
        });
    window.run(event_loop, |event| match event {
        WindowEvents::Resized { width, height } => {
            context.resize(width, height);
            window.request_redraw();
        }
        WindowEvents::Draw => {
            intro.scene.render(&["intro"], &context).unwrap();
        }
        _ => {}
    });
}
