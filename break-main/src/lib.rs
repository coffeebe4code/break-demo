use engine::{
    context::Context,
    description::TextureDescription,
    font::{Font, FontColor},
    vertex::vertex2dtexture,
    window::{Window, WindowEvents},
};
use render_logic::IntroContainer;
use winit::event_loop::EventLoop;

pub async fn main_work() -> () {
    let event_loop = EventLoop::new().unwrap();
    let window = Window::new(&event_loop, "Puzzle Break!");
    let mut context = Context::new(&window).await;
    let mut intro = IntroContainer::new(&context);

    //let verts = vec![
    //    vertex2d([-1.0, 1.0]),
    //    vertex2d([1.0, 1.0]),
    //    vertex2d([1.0, -1.0]),
    //    vertex2d([1.0, -1.0]),
    //    vertex2d([-1.0, -1.0]),
    //    vertex2d([-1.0, 1.0]),
    //];
    let coords = vec![
        vertex2dtexture([0.0, 0.0], [0, 1]),
        vertex2dtexture([0.15, 0.0], [1, 1]),
        vertex2dtexture([0.15, 0.15], [1, 0]),
        vertex2dtexture([0.0, 0.15], [0, 0]),
        vertex2dtexture([0.15, 0.0], [0, 1]),
        vertex2dtexture([0.30, 0.0], [1, 1]),
        vertex2dtexture([0.30, 0.15], [1, 0]),
        vertex2dtexture([0.15, 0.15], [0, 0]),
        vertex2dtexture([0.30, 0.0], [0, 1]),
        vertex2dtexture([0.45, 0.0], [1, 1]),
        vertex2dtexture([0.45, 0.15], [1, 0]),
        vertex2dtexture([0.30, 0.15], [0, 0]),
        vertex2dtexture([0.45, 0.0], [0, 1]),
        vertex2dtexture([0.60, 0.0], [1, 1]),
        vertex2dtexture([0.60, 0.15], [1, 0]),
        vertex2dtexture([0.45, 0.15], [0, 0]),
        vertex2dtexture([0.60, 0.0], [0, 1]),
        vertex2dtexture([0.75, 0.0], [1, 1]),
        vertex2dtexture([0.75, 0.15], [1, 0]),
        vertex2dtexture([0.60, 0.15], [0, 0]),
        vertex2dtexture([0.75, 0.0], [0, 1]),
        vertex2dtexture([0.90, 0.0], [1, 1]),
        vertex2dtexture([0.90, 0.15], [1, 0]),
        vertex2dtexture([0.75, 0.15], [0, 0]),
    ];
    let indices_b = &[0, 1, 2, 2, 3, 0];
    let indices_r = &[4, 5, 6, 6, 7, 4];
    let indices_e = &[8, 9, 10, 10, 11, 8];
    let indices_a = &[12, 13, 14, 14, 15, 12];
    let indices_k = &[16, 17, 18, 18, 19, 16];
    let indices_i = &[20, 21, 22, 22, 23, 20];
    intro.scene.update_verticies("intro", &context, &coords);
    //intro.scene.update_verticies("background", &context, &verts);

    intro.scene.update("intro", "b", &context, &mut |c, d| {
        let desc: &mut TextureDescription =
            d.as_any().downcast_mut::<TextureDescription>().unwrap();
        desc.update(c, indices_b);
    });
    intro.scene.update("intro", "r", &context, &mut |c, d| {
        let desc: &mut TextureDescription =
            d.as_any().downcast_mut::<TextureDescription>().unwrap();
        desc.update(c, indices_r);
    });
    intro.scene.update("intro", "e", &context, &mut |c, d| {
        let desc: &mut TextureDescription =
            d.as_any().downcast_mut::<TextureDescription>().unwrap();
        desc.update(c, indices_e);
    });
    intro.scene.update("intro", "a", &context, &mut |c, d| {
        let desc: &mut TextureDescription =
            d.as_any().downcast_mut::<TextureDescription>().unwrap();
        desc.update(c, indices_a);
    });
    intro.scene.update("intro", "k", &context, &mut |c, d| {
        let desc: &mut TextureDescription =
            d.as_any().downcast_mut::<TextureDescription>().unwrap();
        desc.update(c, indices_k);
    });
    intro.scene.update("intro", "i", &context, &mut |c, d| {
        let desc: &mut TextureDescription =
            d.as_any().downcast_mut::<TextureDescription>().unwrap();
        desc.update(c, indices_i);
    });
    window.run(event_loop, |event| match event {
        WindowEvents::Resized { width, height } => {
            context.resize(width, height);
            intro.scene.update("intro", "font", &context, &mut |c, d| {
                let desc: &mut Font = d.as_any().downcast_mut::<Font>().unwrap();
                desc.update(c, FontColor::new(255, 255, 255), (400.0, 300.0));
            });
            window.request_redraw();
        }
        WindowEvents::Draw => {
            intro.scene.render(&["intro"], &context).unwrap();
            window.request_redraw();
        }
        _ => {}
    });
}
