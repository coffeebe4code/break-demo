use engine::{
    context::Context,
    vertex::vertex,
    window::{Window, WindowEvents},
};
use render_logic::IntroContainer;
use winit::event_loop::EventLoop;

pub async fn main_work() -> () {
    let event_loop = EventLoop::new().unwrap();
    let window = Window::new(&event_loop);
    let mut context = Context::new(&window).await;
    let mut container = IntroContainer::new(&context);

    let indices_b = &[0, 1, 2, 2, 3, 0];
    let b = vec![
        vertex([0.0, 0.0], [0, 1]),
        vertex([0.15, 0.0], [1, 1]),
        vertex([0.15, 0.15], [1, 0]),
        vertex([0.0, 0.15], [0, 0]),
    ];
    let indices_r = &[0, 1, 2, 2, 3, 0];
    let r = vec![
        vertex([0.15, 0.15], [0, 1]),
        vertex([0.30, 0.15], [1, 1]),
        vertex([0.30, 0.30], [1, 0]),
        vertex([0.15, 0.30], [0, 0]),
    ];
    container.update("intro", "b description", &b, indices_b, &context);
    container.update("intro", "r description", &r, indices_r, &context);
    window.run(event_loop, |event| match event {
        WindowEvents::Resized { width, height } => {
            context.resize(width, height);
            window.request_redraw();
        }
        WindowEvents::Draw => {
            container.render(&["intro"], &context).unwrap();
        }
        _ => {}
    });
}
