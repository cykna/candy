pub mod elements;
pub mod helpers;
pub mod renderer;
pub mod text;
pub mod ui;

use elements::{
    CandyElement, CandySquare,
    text::{CandyText, TextAlignment},
};
use nalgebra::Vector2;
use renderer::{CandyRenderer, candy::CandyDefaultRenderer, twod::BiDimensionalRenderer};

use skia_safe::{FontMgr, FontStyle};
use text::font::CandyFont;
use winit::{
    dpi::PhysicalSize,
    event::MouseButton,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

#[cfg(feature = "opengl")]
pub use glutin::config::Config;

pub trait CandyHandler {
    #[cfg(feature = "opengl")]
    fn new(window: Window, config: Config) -> Self;
    fn draw(&mut self);
    fn resize(&mut self, size: PhysicalSize<u32>);
    fn on_mouse_move(&mut self, position: Vector2<f32>);
    fn on_press(&mut self, button: MouseButton);
    fn exit(&self);
}

#[derive(Default, Debug)]
struct CandyWindow<T> {
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
                winit::event::WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                } => {
                    if state.is_pressed() {
                        handler.on_press(button)
                    }
                }
                winit::event::WindowEvent::CursorMoved {
                    device_id,
                    position,
                } => {
                    handler.on_mouse_move(Vector2::new(position.x as f32, position.y as f32));
                }
                _ => {}
            }
        }
    }
}

pub struct CandyDefaultHandler {
    mouse_pos: Vector2<f32>,
    window: Window,
    renderer: CandyDefaultRenderer,
    element: CandyElement,
    img: CandyElement,
}

impl CandyHandler for CandyDefaultHandler {
    fn new(window: Window, config: Config) -> Self {
        use nalgebra::{Vector2, Vector4};
        let tf = FontMgr::new()
            .legacy_make_typeface(Some("Inter"), FontStyle::default())
            .unwrap();
        Self {
            mouse_pos: Vector2::new(0.0, 0.0),
            img: CandyElement::new_text({
                CandyText::new(
                    "pedro",
                    Vector2::new(50.0, 100.0),
                    CandyFont::new(tf, 18.0),
                    Vector4::new(1.0, 1.0, 0.0, 1.0),
                    TextAlignment::Center,
                )
            }),
            renderer: CandyDefaultRenderer::new(&window, &config),
            window,
            element: CandyElement::new_square(CandySquare::new(
                Vector2::new(50.0, 50.0),
                Vector2::new(50.0, 100.0),
                Vector4::new(1.0, 0.0, 1.0, 0.5),
                Some(Vector4::new(1.0, 0.0, 0.0, 1.0)),
                Some(Vector2::new(5.0, 5.0)),
            )),
        }
    }
    fn on_mouse_move(&mut self, position: Vector2<f32>) {
        self.mouse_pos = position;
    }
    fn on_press(&mut self, button: MouseButton) {}
    fn draw(&mut self) {
        //self.element
        //    .render(self.renderer.twod_renderer().twod_painter());
        self.img
            .render(self.renderer.twod_renderer().twod_painter());
        self.renderer.flush();
    }
    fn resize(&mut self, size: PhysicalSize<u32>) {
        #[cfg(feature = "opengl")]
        self.renderer.resize(&self.window, size.width, size.height);
    }
    fn exit(&self) {}
}

fn main() {
    CandyWindow::<CandyDefaultHandler>::new(
        Window::default_attributes()
            .with_transparent(true)
            .with_title("Candy"),
    )
    .run();
}
