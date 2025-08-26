pub mod components;
pub mod elements;
pub mod handler;
pub mod helpers;
pub mod renderer;
pub mod text;
pub mod ui;
pub mod window;

use elements::CandySquare;
use helpers::rect::Rect;
use nalgebra::{Vector2, Vector4};
use renderer::twod::BiDimensionalPainter;

use ui::{
    component::{Component, ComponentRenderer, RootComponent},
    styling::fx::Shadow,
    styling::{self, layout::Size},
};
use window::CandyWindow;
use winit::{event::MouseButton, window::Window};

#[cfg(feature = "opengl")]
pub use glutin::config::Config;

use crate::{
    elements::text::CandyText,
    text::{font::CandyFont, manager::FontManager},
    ui::styling::{fx::Effect, style::Style},
};

pub enum Msg {
    None,
    MarkUndirty,
    Write(String),
}

pub struct Square {
    text: CandyText,
    info: CandySquare,
}

pub struct Red;

impl Style for Red {
    fn color(&self) -> Vector4<f32> {
        Vector4::new(1.0, 0.0, 0.0, 1.0)
    }
    fn border_color(&self) -> Vector4<f32> {
        Vector4::new(0.0, 1.0, 0.0, 1.0)
    }
    fn border_width(&self) -> f32 {
        1.0
    }

    fn border_radius(&self) -> Vector2<f32> {
        Vector2::new(5.0, 5.0)
    }

    fn effect(&self) -> impl Effect + 'static {
        Shadow::colored((self.border_color() + Vector4::new(1.0, 1.0, 1.0, 1.0)) * 0.5)
            .with_blur(Vector2::new(10.0, 10.0))
    }
}

impl Square {
    pub fn new(font: CandyFont) -> Self {
        Self {
            text: CandyText::new("pedro", Vector2::zeros(), font).with_style(&Red),
            info: CandySquare::new(Vector2::zeros(), Vector2::zeros()).with_style(&Red),
        }
    }
}

impl Component for Square {
    fn resize(&mut self, rect: Rect) {
        if rect
            != (Rect {
                x: self.info.position().x,
                y: self.info.position().y,
                width: self.info.size().x,
                height: self.info.size().y,
            })
        {
            self.info.position_mut().x = rect.x;
            self.info.position_mut().y = rect.y;

            self.info.size_mut().x = rect.width;
            self.info.size_mut().y = rect.height;
            self.text.position_mut().x = rect.x;
            self.text.position_mut().y = rect.y;
        }
    }

    fn render(&self, renderer: &mut ComponentRenderer) {
        renderer.square(&self.info);
        renderer.text(&self.text);
    }
}

#[derive(Default)]
struct State {
    w: f32,
    h: f32,
    data: f32,
    squares: Vec<Square>,
    manager: FontManager,
}
impl State {
    fn resize_children(&mut self) {
        let mut style = styling::layout::Layout::default()
            .with_corner(styling::layout::Corner::TopLeft)
            .with_direction(styling::layout::Direction::Vertical)
            .with_gap(Vector2::new(Size::Length(5.0), Size::Length(10.0)))
            .with_padding(Vector4::new(
                Size::Length(5.0),
                Size::Length(50.0),
                Size::Length(5.0),
                Size::Length(10.0),
            ));
        for _ in &self.squares {
            style = style.with_definition(styling::layout::DefinitionRect {
                x: Size::Length(0.0),
                y: Size::Length(0.0),
                width: Size::Percent(0.25),
                height: Size::Percent(0.25),
            });
        }
        for (idx, r) in style
            .calculate(Rect {
                x: 0.0,
                y: 0.0,
                width: self.w,
                height: self.h,
            })
            .into_iter()
            .enumerate()
        {
            self.squares[idx].resize(r);
        }
    }
}

impl Component for State {
    fn resize(&mut self, rect: Rect) {
        self.w = rect.width;
        self.h = rect.height;
        self.resize_children();
    }
    fn render(&self, renderer: &mut ComponentRenderer) {
        renderer.background(&Vector4::new(0.0, 0.1, 0.2, 1.0));
        for s in &self.squares {
            s.render(renderer);
        }
    }
}

impl RootComponent for State {
    fn new() -> Self {
        Self {
            w: 0.0,
            h: 0.0,
            data: 0.0,
            squares: Vec::new(),
            manager: FontManager::new(),
        }
    }
    fn click(&mut self, _: Vector2<f32>, _: MouseButton) -> bool {
        self.data += 0.1;

        let s = Square::new(self.manager.create_font("Nimbus Roman", 24.0));

        self.squares.push(s);

        self.resize_children();
        true
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
