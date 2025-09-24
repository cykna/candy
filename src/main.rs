pub mod components;
pub mod elements;
pub mod handler;
pub mod helpers;
pub mod renderer;
pub mod text;
pub mod ui;
pub mod window;

use crate::components::Input;
use crate::components::{Scrollable, ScrollableConfig};
use crate::ui::styling::layout::Layout;
use crate::ui::styling::layout::{DefinitionRect, Direction};
use crate::{components::Button, ui::styling::fx::Effect};

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
use winit::keyboard::Key;
use winit::{event::MouseButton, window::Window};

#[cfg(feature = "opengl")]
pub use glutin::config::Config;

use crate::{
    components::Text,
    elements::text::CandyText,
    text::{font::CandyFont, manager::FontManager},
    ui::styling::style::Style,
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

    fn effect(&self) -> Box<dyn Effect> {
        Box::new(
            Shadow::colored((self.border_color() + Vector4::new(1.0, 1.0, 1.0, 1.0)) * 0.5)
                .with_blur(Vector2::new(10.0, 10.0)),
        )
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

    fn apply_style(&mut self, _: &dyn Style) {}
}

struct State {
    w: f32,
    h: f32,
    data: Scrollable,
    input: Input,
    manager: FontManager,
}
impl State {
    fn resize_children(&mut self) {
        let mut style = Layout::vertical();
        style
            .with_corner(styling::layout::Corner::TopLeft)
            .with_direction(styling::layout::Direction::Vertical)
            .with_gap(Vector2::new(Size::Length(5.0), Size::Length(10.0)))
            .with_padding(Vector4::new(
                Size::Length(5.0),
                Size::Length(50.0),
                Size::Length(5.0),
                Size::Length(10.0),
            ))
            .with_definition(styling::layout::DefinitionRect {
                x: Size::Length(0.0),
                y: Size::Length(0.0),
                width: Size::Percent(0.25),
                height: Size::Length(50.0),
            });
    }
}

impl Component for State {
    fn resize(&mut self, rect: Rect) {
        self.w = rect.width;
        self.h = rect.height;
        self.data.resize(rect);
    }
    fn render(&self, renderer: &mut ComponentRenderer) {
        renderer.background(&Vector4::new(0.0, 0.1, 0.2, 1.0));
        self.data.render(renderer);
        self.input.render(renderer);
    }
    fn apply_style(&mut self, _: &dyn Style) {}
}

pub struct RedShadow;
impl Style for RedShadow {
    fn effect(&self) -> Box<dyn crate::ui::styling::fx::Effect> {
        Box::new(RedShadow)
    }
    fn background_color(&self) -> Vector4<f32> {
        Vector4::new(0.0, 1.0, 1.0, 0.5)
    }
    fn color(&self) -> Vector4<f32> {
        Vector4::new(1.0, 1.0, 0.0, 1.0)
    }
}

impl Effect for RedShadow {
    fn shadow(&self) -> Option<crate::ui::styling::fx::ShadowEffect> {
        Some(crate::ui::styling::fx::ShadowEffect {
            color: Vector4::new(1.0, 0.0, 0.0, 1.0),
            offset: Vector2::new(20.0, 20.0),
            blur: Vector2::new(10.0, 10.0),
        })
    }
}

pub struct InputStyle;
impl Style for InputStyle {
    fn color(&self) -> Vector4<f32> {
        Vector4::new(1.0, 1.0, 1.0, 1.0)
    }
    fn background_color(&self) -> Vector4<f32> {
        Vector4::new(0.0, 1.0, 1.0, 0.7)
    }
    fn effect(&self) -> Box<dyn crate::ui::styling::fx::Effect> {
        Box::new(RedShadow)
    }
}

impl RootComponent for State {
    fn new() -> Self {
        let font = FontManager::new();
        let content = font.create_font("Fira Sans", 24.0);
        Self {
            w: 0.0,
            h: 0.0,
            input: {
                let mut inp = Input::new(Text::new_content("Pascal", content.clone()));
                inp.apply_style(&InputStyle);
                inp
            },
            data: {
                let mut scroll = Scrollable::new(ScrollableConfig {
                    layout: Layout::vertical(),
                    scroll_bar_width: 10.0,
                    direction: Direction::Vertical,
                });
                scroll.apply_style_scrollbar(&RedShadow);
                scroll
            },
            manager: font,
        }
    }

    fn keydown(
        &mut self,
        key: winit::keyboard::Key<winit::keyboard::SmolStr>,
        loc: winit::keyboard::KeyLocation,
    ) -> bool {
        match key {
            Key::Character(c) => {
                self.input.write_str(&c);
                true
            }
            Key::Named(key) => {
                if let winit::keyboard::NamedKey::ArrowLeft = key {
                    self.input.move_left(1);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn keyup(
        &mut self,
        _: winit::keyboard::Key<winit::keyboard::SmolStr>,
        _: winit::keyboard::KeyLocation,
    ) -> bool {
        false
    }
    fn click(&mut self, pos: Vector2<f32>, btn: MouseButton) -> bool {
        self.data.on_mouse(pos);

        let font = self.manager.create_font("Nimbus Roman", 24.0);
        let s = Button::new(Text::new_content("Hello World", font), move |pos, btn| {
            Msg::None
        })
        .with_style(&RedShadow);

        self.data.add_child(
            s,
            DefinitionRect {
                x: Size::Length(0.0),
                y: Size::Length(10.0),
                width: Size::Percent(0.25),
                height: Size::Percent(0.25),
            },
        );

        println!(
            "{:?} {}",
            self.data.children().len(),
            self.data.is_dragging()
        );

        self.resize(Rect {
            x: 0.0,
            y: 0.0,
            width: self.w,
            height: self.h,
        });
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
