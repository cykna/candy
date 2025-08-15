pub mod elements;
pub mod helpers;
pub mod renderer;
pub mod text;
pub mod ui;

use elements::{CandyElement, CandySquare};
use nalgebra::{Vector2, Vector4};
use renderer::{
    CandyRenderer,
    candy::CandyDefaultRenderer,
    twod::{BiDimensionalPainter, BiDimensionalRenderer, Candy2DRenderer},
};

use skia_safe::{FontMgr, FontStyle};
use taffy::{LengthPercentageAuto, Size, Style, geometry::Rect};
use text::font::CandyFont;
use ui::{
    component::{Component, ComponentRenderer},
    tree::{
        node::{CandyKey, CandyNode, ElementBuilder},
        tree::CandyTree,
    },
};
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
    ui: CandyTree,
    state: f32,
}

struct P {}

struct S {
    node: CandyNode<ComponentRenderer>,
    children: Vec<Box<dyn Component>>,
}

impl Component for S {
    fn new(tree: &mut CandyTree) -> Self
    where
        Self: Sized,
    {
        Self {
            node: ElementBuilder::square(CandySquare::new(
                Vector2::new(0.0, 0.0),
                Vector2::new(0.0, 0.0),
                Vector4::new(1.0, 1.0, 0.0, 1.0),
                None,
                None,
            ))
            .classed("pedro")
            .build(tree),
            children: Vec::new(),
        }
    }
    fn inner(&self) -> &CandyNode<ComponentRenderer> {
        &self.node
    }
    fn inner_mut(&mut self) -> &mut CandyNode<ComponentRenderer> {
        &mut self.node
    }
    fn resize(&mut self, layout: &ui::layout::CandyLayout) {
        let layout = layout.layout_of(self.node.layout()).unwrap();
        self.node.resize(layout);
    }

    fn render(&self, painter: &mut ComponentRenderer) {
        self.node.render(painter);
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Component>> {
        &mut self.children
    }
}

impl CandyHandler for CandyDefaultHandler {
    fn new(window: Window, config: Config) -> Self {
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
    }
    fn on_press(&mut self, button: MouseButton) {
        self.state += 0.5;
        self.ui.append_component::<S>();
        self.renderer
            .twod_renderer()
            .twod_painter()
            .background(&Vector4::new(0.0, 0.0, 0.0, 0.0));

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
