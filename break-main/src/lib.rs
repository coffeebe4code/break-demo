use engine::{
    context::Context,
    empty::EmptyPass,
    pass::{Pass, Passable},
    texture::Texture,
    vertex::vertex,
    window::{Window, WindowEvents},
};
use winit::event_loop::EventLoop;

pub async fn main_work() -> () {
    let event_loop = EventLoop::new().unwrap();
    let window = Window::new(&event_loop);
    let mut context = Context::new(&window).await;
    window.request_redraw();
    window.run(event_loop, |event| match event {
        WindowEvents::Resized { width, height } => {
            context.resize(width, height);
        }
        WindowEvents::Draw => {
            EmptyPass::render(&context).unwrap();
        }
        _ => {}
    });
}
