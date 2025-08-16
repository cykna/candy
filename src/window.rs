use nalgebra::Vector2;
use winit::{event_loop::EventLoop, window::WindowAttributes};

use crate::handler::CandyHandler;

#[derive(Default, Debug)]
pub struct CandyWindow<T> {
    handler: Option<T>,
    attribs: WindowAttributes,
}
impl<T> CandyWindow<T>
where
    T: CandyHandler,
{
    pub fn new(attribs: WindowAttributes) -> Self {
        Self {
            handler: None,
            attribs,
        }
    }

    pub fn run(&mut self) {
        let lp = EventLoop::new().unwrap();
        #[cfg(feature = "opengl")]
        {
            use glutin::config::{ConfigTemplateBuilder, GlConfig};

            use glutin_winit::DisplayBuilder;
            let template = ConfigTemplateBuilder::new()
                .with_alpha_size(8)
                .with_transparency(true);
            let (window, config) = DisplayBuilder::new()
                .with_window_attributes(Some(self.attribs.clone()))
                .build(&lp, template, |configs| {
                    configs
                        .reduce(|accum, config| {
                            let transparency_check =
                                config.supports_transparency().unwrap_or(false)
                                    && !accum.supports_transparency().unwrap_or(false);
                            if transparency_check || config.num_samples() < accum.num_samples() {
                                config
                            } else {
                                accum
                            }
                        })
                        .unwrap()
                })
                .unwrap();
            self.handler = Some(T::new(window.expect("Window not created??"), config));
        };
        lp.run_app(self).unwrap();
    }
}

impl<T> winit::application::ApplicationHandler for CandyWindow<T>
where
    T: CandyHandler,
{
    fn resumed(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        #[cfg(not(feature = "opengl"))]
        println!("gayzinho");
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(ref mut handler) = self.handler {
            match event {
                winit::event::WindowEvent::RedrawRequested => {
                    handler.draw();
                }
                winit::event::WindowEvent::Resized(size) => {
                    handler.resize(size);
                }
                winit::event::WindowEvent::CloseRequested => {
                    event_loop.exit();
                    handler.exit();
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    if state.is_pressed() {
                        handler.on_press(button)
                    }
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    handler.on_mouse_move(Vector2::new(position.x as f32, position.y as f32));
                }
                _ => {}
            }
        }
    }
}
