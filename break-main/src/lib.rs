use engine::{
    context::Context,
    passmanager::{Pass, Passable},
    texture::Texture,
    vertex::vertex,
    window::{Window, WindowEvents},
};
use winit::event_loop::EventLoop;

pub async fn main_work() -> () {
    let event_loop = EventLoop::new().unwrap();
    let window = Window::new(&event_loop);
    let mut context = Context::new(&window).await;
    let attributes = vec![
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x2,
        },
    ];
    let mut pass = Pass::new(&context.device, &context.config, &attributes);

    let indices_pb = &[0, 1, 2, 2, 3, 0];
    let diffuse_bytes_pb = include_bytes!("../../assets/PowerBorder.png");
    let texture_pb = Texture::from_bytes(
        &context.device,
        &context.queue,
        diffuse_bytes_pb,
        "PowerBorder",
    )
    .unwrap();
    let pb = vec![
        vertex([-0.10, -0.10, 0.0], [0, 1]),
        vertex([-0.20, -0.10, 0.0], [1, 1]),
        vertex([-0.20, -1.00, 0.0], [1, 0]),
        vertex([-0.10, -1.00, 0.0], [0, 0]),
    ];
    pass.add(indices_pb, &pb, texture_pb, &context.device);

    let indices_b = &[0, 1, 2, 2, 3, 0];
    let diffuse_bytes_b = include_bytes!("../../assets/B.png");
    let texture_b =
        Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_b, "B").unwrap();
    let b = vec![
        vertex([0.0, 0.0, 0.0], [0, 1]),
        vertex([0.15, 0.0, 0.0], [1, 1]),
        vertex([0.15, 0.15, 0.0], [1, 0]),
        vertex([0.0, 0.15, 0.0], [0, 0]),
    ];
    pass.add(indices_b, &b, texture_b, &context.device);

    let indices_r = &[0, 1, 2, 2, 3, 0];
    let diffuse_bytes_r = include_bytes!("../../assets/R.png");
    let texture_r =
        Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_r, "R").unwrap();
    let r = vec![
        vertex([0.15, 0.0, 0.0], [0, 1]),
        vertex([0.30, 0.0, 0.0], [1, 1]),
        vertex([0.30, 0.15, 0.0], [1, 0]),
        vertex([0.15, 0.15, 0.0], [0, 0]),
    ];
    pass.add(indices_r, &r, texture_r, &context.device);

    let indices_e = &[0, 1, 2, 2, 3, 0];
    let diffuse_bytes_e = include_bytes!("../../assets/E.png");
    let texture_e =
        Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_e, "E").unwrap();
    let e = vec![
        vertex([0.30, 0.0, 0.0], [0, 1]),
        vertex([0.45, 0.0, 0.0], [1, 1]),
        vertex([0.45, 0.15, 0.0], [1, 0]),
        vertex([0.30, 0.15, 0.0], [0, 0]),
    ];
    pass.add(indices_e, &e, texture_e, &context.device);

    let indices_a = &[0, 1, 2, 2, 3, 0];
    let diffuse_bytes_a = include_bytes!("../../assets/A.png");
    let texture_a =
        Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_a, "A").unwrap();
    let a = vec![
        vertex([0.45, 0.0, 0.0], [0, 1]),
        vertex([0.60, 0.0, 0.0], [1, 1]),
        vertex([0.60, 0.15, 0.0], [1, 0]),
        vertex([0.45, 0.15, 0.0], [0, 0]),
    ];
    pass.add(indices_a, &a, texture_a, &context.device);

    let indices_k = &[0, 1, 2, 2, 3, 0];
    let diffuse_bytes_k = include_bytes!("../../assets/K.png");
    let texture_k =
        Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_k, "K").unwrap();
    let k = vec![
        vertex([0.60, 0.0, 0.0], [0, 1]),
        vertex([0.75, 0.0, 0.0], [1, 1]),
        vertex([0.75, 0.15, 0.0], [1, 0]),
        vertex([0.60, 0.15, 0.0], [0, 0]),
    ];
    pass.add(indices_k, &k, texture_k, &context.device);

    let indices_x = &[0, 1, 2, 2, 3, 0];
    let diffuse_bytes_x = include_bytes!("../../assets/I.png");
    let texture_x =
        Texture::from_bytes(&context.device, &context.queue, diffuse_bytes_x, "I").unwrap();
    let x = vec![
        vertex([0.75, 0.0, 0.0], [0, 1]),
        vertex([0.90, 0.0, 0.0], [1, 1]),
        vertex([0.90, 0.15, 0.0], [1, 0]),
        vertex([0.75, 0.15, 0.0], [0, 0]),
    ];
    pass.add(indices_x, &x, texture_x, &context.device);

    pass.build(&context.device);
    let mut surface_ready = false;
    window.run(event_loop, |event| match event {
        WindowEvents::Resized { width, height } => {
            surface_ready = true;

            context.resize(width, height);
        }
        WindowEvents::Draw => {
            if !surface_ready {
                return;
            }
            window.request_redraw();
            match pass.render(&context.surface, &context.device, &context.queue) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    context.resize(900, 900);
                }
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    panic!("oom");
                }

                // This happens when the a frame takes too long to present
                Err(wgpu::SurfaceError::Timeout) => {
                    panic!("surface timeout");
                }
                _ => {
                    panic!("oooh geez");
                }
            }
        }
        _ => {}
    });
}
