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
    container.update("intro", "b entries", &b, indices_b, &context);
    let mut surface_ready = false;
    window.run(event_loop, |event| match event {
        WindowEvents::Resized { width, height } => {
            context.resize(width, height);
            println!("resized");
            surface_ready = true;
            window.request_redraw();
        }
        WindowEvents::Draw => {
            if !surface_ready {
                return;
            }
            container.render("intro", &context).unwrap()
        }
        _ => {}
    });
}
