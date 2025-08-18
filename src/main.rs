pub mod elements;
pub mod handler;
pub mod helpers;
pub mod renderer;
pub mod text;
pub mod ui;
pub mod window;

use elements::CandySquare;
use handler::{CandyDefaultHandler, CandyHandler};
use helpers::rect::Rect;
use nalgebra::{Vector2, Vector4};
use renderer::{
    CandyRenderer,
    candy::CandyDefaultRenderer,
    twod::{BiDimensionalPainter, BiDimensionalRenderer},
};

use ui::component::{Component, ComponentRenderer, RootComponent};
use window::CandyWindow;
use winit::{dpi::PhysicalSize, event::MouseButton, window::Window};

#[cfg(feature = "opengl")]
pub use glutin::config::Config;

pub enum Msg {
    None,
    Write(String),
}

pub struct Text {
    text: String,
}

pub struct Square {
    info: CandySquare,
}

impl Component for Square {
    type Message = Msg;
    fn resize(&mut self, rect: Rect) {
        self.info.position_mut().x = rect.x;
        self.info.position_mut().y = rect.y;

        self.info.size_mut().x = rect.width;
        self.info.size_mut().y = rect.height;
    }

    fn render(&self, renderer: &mut ComponentRenderer) {
        renderer.square(&self.info);
    }
    fn on_message(&mut self, msg: Msg) -> Msg {
        Msg::None
    }
}

#[derive(Default)]
struct State {
    data: f32,
}

impl RootComponent for State {
    type Message = Msg;
    fn click(&mut self, position: Vector2<f32>, button: MouseButton) -> bool {
        self.data += 0.02;
        self.data < 1.0
    }
    fn resize(&mut self, width: f32, height: f32) {}
    fn render(&self, renderer: &mut ComponentRenderer) {
        println!("Rendering again");
        renderer.square(
            &CandySquare::new(
                Vector2::new(0.0, 0.0),
                Vector2::new(50.0, 50.0),
                Vector4::zeros(),
                None,
                None,
            )
            .with_r(self.data)
            .with_a(1.0),
        );
    }
    fn on_message(&mut self, msg: Self::Message) -> Self::Message {
        Self::Message::None
    }
}

fn main() {
    CandyWindow::<State>::new(
        Window::default_attributes()
            .with_transparent(true)
            .with_title("Candy"),
    )
    .run();
}
