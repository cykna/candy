pub mod elements;
pub mod helpers;
pub mod renderer;
pub mod text;
pub mod ui;

use elements::{
    CandyElement, CandySquare,
    text::{CandyText, TextAlignment},
};
use nalgebra::{Vector, Vector2, Vector4};
use renderer::{
    CandyRenderer,
    candy::CandyDefaultRenderer,
    twod::{BiDimensionalPainter, BiDimensionalRenderer, Candy2DRenderer},
};

use skia_safe::{FontMgr, FontStyle};
use taffy::{Dimension, Size, Style};
use text::font::CandyFont;
use ui::tree::{node::ElementBuilder, tree::CandyTree};
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
    ui: CandyTree<Candy2DRenderer>,
    state: f32,
}
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let h = h as u32 % 360;

    let (r, g, b) = match h {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    (r + m, g + m, b + m)
}

impl CandyHandler for CandyDefaultHandler {
    fn new(window: Window, config: Config) -> Self {
        let tf = FontMgr::new()
            .legacy_make_typeface(Some("Inter"), FontStyle::default())
            .unwrap();
        let mut ui = CandyTree::new(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );
        ui.create_style(
            "pedro",
            Style {
                size: Size::from_percent(0.5, 0.5),
                ..Default::default()
            },
        );
        Self {
            state: 0.0,
            ui,
            mouse_pos: Vector2::new(0.0, 0.0),
            renderer: CandyDefaultRenderer::new(&window, &config),
            window,
        }
    }
    fn on_mouse_move(&mut self, position: Vector2<f32>) {
        self.mouse_pos = position;
        self.renderer
            .twod_renderer()
            .twod_painter()
            .background(&Vector4::new(0.0, 0.0, 0.0, 0.0));
        self.state += 1.0;
        self.window.request_redraw();
    }
    fn on_press(&mut self, button: MouseButton) {
        self.ui.clear();
        self.ui.append_root(
            ElementBuilder::square(CandySquare::new(
                Vector2::new(50.0, 50.0),
                Vector2::new(20.0, 20.0),
                {
                    let (r, g, b) = hsv_to_rgb(self.state, 1.0, 1.0);
                    Vector4::new(r, g, b, 1.0)
                },
                None,
                None,
            ))
            .children(vec![])
            .styled("pedro"),
        );
        self.window.request_redraw();
    }
    fn draw(&mut self) {
        self.ui.render(self.renderer.twod_renderer());
        self.renderer.flush();
    }

    #[cfg(feature = "opengl")]
    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.renderer.resize(&self.window, size.width, size.height);
        self.ui.resize(size.width as f32, size.height as f32);
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
