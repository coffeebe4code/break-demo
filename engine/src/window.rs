use std::{
    fs::{self, File},
    io::Read,
    sync::Arc,
};

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Icon, WindowBuilder},
};

pub enum WindowEvents {
    Resized {
        width: u32,
        height: u32,
    },
    ScaleFactorChanged {
        scale_factor: f64,
        writer: InnerSizeWriter,
    },
    Keyboard {
        state: ElementState,
        key: Option<KeyCode>,
    },
    Draw,
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Window {
    pub window: Arc<winit::window::Window>,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }
        //let mut f = File::open("./assets/B.png").expect("texture not found");
        //let metadata = fs::metadata("./assets/B.png").expect("unable to read metadata");
        //let mut buffer = vec![0; metadata.len() as usize];
        //f.read(&mut buffer).expect("buffer overflow");
        //let img = image::load_from_memory(&buffer).unwrap();
        let window = WindowBuilder::new()
            .with_title("Break! Game")
            .with_maximized(false)
            //.with_window_icon(Some(
            //    Icon::from_rgba(img.to_rgb8().into_raw(), 20, 20).unwrap(),
            //))
            .build(&event_loop)
            .unwrap();

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            let size = window.inner_size();
            let _ = window.request_inner_size(PhysicalSize::new(900, 900));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("break-canvas")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
            }
        }

        Self {
            window: Arc::new(window),
        }
    }
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
    pub fn run(&self, event_loop: EventLoop<()>, mut callback: impl FnMut(WindowEvents) -> ()) {
        event_loop
            .run(|event, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::Resized(physical_size) => callback(WindowEvents::Resized {
                        width: physical_size.width,
                        height: physical_size.height,
                    }),
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        inner_size_writer,
                    } => callback(WindowEvents::ScaleFactorChanged {
                        scale_factor: *scale_factor,
                        writer: inner_size_writer.clone(),
                    }),

                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => control_flow.exit(),
                    WindowEvent::RedrawRequested => callback(WindowEvents::Draw),
                    _ => {}
                },
                _ => {}
            })
            .unwrap();
    }
}
