use std::iter;

use image::DynamicImage;
#[cfg(target_arch = "wasm32")]
use web_sys::console;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod graphics;
mod hardware;
mod runtime;
mod texture;
mod audio;
mod util;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::Builder::new()
                .filter_level(log::LevelFilter::Warn)
                .init();
        }
    }
    log::warn!("USER::: Starting");
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        let _ = window.request_inner_size(PhysicalSize::new(1280, 640));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    // State::new uses async code, so we're going to wait for it to finish
    let state = graphics::State::new(&window).await;
    let mut runtime = runtime::Runtime::new(state).await;
    let mut surface_configured = false;

    event_loop
        .run( move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == runtime.graphics.window().id() => {
                    match event {
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
                        WindowEvent::KeyboardInput {
                            event: key_event @ KeyEvent{
                                repeat: false,..
                            }, ..
                        } => {
                            runtime.input(key_event);
                        }
                        WindowEvent::Resized(physical_size) => {
                            surface_configured = true;
                            runtime.graphics.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            if !surface_configured {
                                return;
                            }
                            runtime.graphics.call_fast_render().unwrap_or_else(|_| {
                                control_flow.exit();
                            });
                        }
                        _ => {}
                    }
                }
                Event::AboutToWait => {
                    runtime.run()
                }
                _ => {}
            }
        })
        .unwrap();
}
