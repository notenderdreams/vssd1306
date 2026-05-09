use std::sync::Arc;

use super::renderer::RendererState;
use crate::display::Display;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
};

pub fn run<F>(mut display: Display, title: &str, mut update: F)
where
    F: FnMut(&mut Display) -> bool + 'static,
{
    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                display.width(),
                display.height(),
            ))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap(),
    );

    let mut renderer = RendererState::new(window.clone(), display.width(), display.height());

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        // Upload 
                        let buffer = display.visible_buffer();
                        let width = display.width();
                        let height = display.height();
                        renderer.update_texture(buffer, width, height);

                        // Draw
                        match renderer.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {
                                renderer.resize(renderer.config.width, renderer.config.height);
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => eprintln!("render error: {:?}", e),
                        }
                    }
                    WindowEvent::Resized(new_size) => {
                        renderer.resize(new_size.width, new_size.height);
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    if !update(&mut display) {
                        elwt.exit();
                    }
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
}
