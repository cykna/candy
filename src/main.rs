pub mod elements;
pub mod handler;
pub mod helpers;
pub mod renderer;
pub mod text;
pub mod ui;
pub mod window;

use std::ops::{Deref, DerefMut};

use elements::CandySquare;
use handler::CandyHandler;
use nalgebra::{Vector2, Vector4};
use renderer::{
    CandyRenderer,
    candy::CandyDefaultRenderer,
    twod::{BiDimensionalPainter, BiDimensionalRenderer},
};
use taffy::{NodeId, Size, Style};
use ui::{
    component::{Component, ComponentRenderer},
    tree::{
        node::{CandyKey, CandyNode, ElementBuilder},
        tree::CandyTree,
    },
};
use window::CandyWindow;
use winit::{dpi::PhysicalSize, event::MouseButton, window::Window};

#[cfg(feature = "opengl")]
pub use glutin::config::Config;

enum Msg {
    None,
    Write(String),
}

pub struct Square {
    node_id: NodeId,
    children: Vec<CandyKey>,
    parent: Option<CandyKey>,
}

impl Component<Msg> for Square {
    fn new(tree: &mut CandyTree<Msg>, parent: Option<CandyKey>) -> Self
    where
        Self: Sized,
    {
        Self {
            parent,
            node_id: tree.use_style("pedro".into(), tree.style_id_of(parent)),
            children: Vec::new(),
        }
    }

    fn layout(&self) -> NodeId {
        self.node_id
    }

    fn parent(&self) -> Option<CandyKey> {
        self.parent
    }

    fn children(&self) -> &Vec<CandyKey> {
        &self.children
    }

    fn render(&self, ui: &CandyTree<Msg>) -> CandyNode<ComponentRenderer> {
        let layout = ui.layout_of(self.layout()).unwrap();
        let pos = Vector2::new(layout.location.x, layout.location.y);
        let color = Vector2::new(layout.size.width, layout.size.height);
        ElementBuilder::square(CandySquare::new(
            pos,
            color,
            Vector4::new(1.0, 1.0, 1.0, 1.0),
            None,
            None,
        ))
        .build()
    }

    fn on_message(&mut self, _: Msg) -> Msg {
        Msg::None
    }
}

pub struct CandyDefaultHandler<M> {
    mouse_pos: Vector2<f32>,
    window: Window,
    renderer: CandyDefaultRenderer,
    ui: CandyTree<M>,
}

impl CandyHandler for CandyDefaultHandler<Msg> {
    fn new(window: Window, config: Config) -> Self {
        let mut ui: CandyTree<Msg> = CandyTree::new(
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
            ui,
            mouse_pos: Vector2::new(0.0, 0.0),
            renderer: CandyDefaultRenderer::new(&window, &config),
            window,
        }
    }
    fn on_mouse_move(&mut self, position: Vector2<f32>) {
        self.mouse_pos = position;
    }
    fn on_press(&mut self, _: MouseButton) {
        self.renderer
            .twod_renderer()
            .twod_painter()
            .background(&Vector4::new(0.0, 0.0, 0.0, 0.0));
        self.ui.append_component::<Square>(None);
        self.window.request_redraw();
    }
    fn draw(&mut self) {
        self.ui.render(self.renderer.twod_renderer());
        self.renderer.flush();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        #[cfg(feature = "opengl")]
        {
            self.renderer.resize(&self.window, size.width, size.height);
        }
        self.ui.resize(size.width as f32, size.height as f32);
    }
    fn exit(&self) {}
}

impl<M> Deref for CandyDefaultHandler<M> {
    type Target = CandyTree<M>;
    fn deref(&self) -> &Self::Target {
        &self.ui
    }
}

impl<M> DerefMut for CandyDefaultHandler<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ui
    }
}

fn main() {
    CandyWindow::<CandyDefaultHandler<Msg>>::new(
        Window::default_attributes()
            .with_transparent(true)
            .with_title("Candy"),
    )
    .run();
}
