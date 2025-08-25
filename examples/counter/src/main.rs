use candy::elements::DrawRule;
use candy::elements::{CandySquare, text::CandyText};
use candy::helpers::rect::Rect;
use candy::nalgebra::{Vector2, Vector4};
use candy::renderer::twod::BiDimensionalPainter;
use candy::text::font::CandyFont;
use candy::ui::component::*;
use candy::ui::styling::{
    fx::{Effect, Shadow},
    layout::{DefinitionRect, Layout, Size},
    style::Style,
};
use candy::{window::CandyWindow, winit::window::WindowAttributes};

#[derive(Default)]
pub struct CounterText {
    text: CandyText,
}

pub struct Blue;

impl Style for Blue {
    fn color(&self) -> Vector4<f32> {
        Vector4::new(1.0, 0.0, 0.0, 1.0)
    }
    fn effect(&self) -> impl Effect + 'static {
        let color = self.color();
        Shadow::new()
            .with_color(Vector4::new(
                1.0 - color.x,
                1.0 - color.y,
                1.0 - color.z,
                1.0,
            ))
            .with_blur(Vector2::new(10.0, 10.0))
    }
    fn border_radius(&self) -> Vector2<f32> {
        Vector2::new(15.0, 15.0)
    }
}

impl Component for CounterText {
    type Message = StateMsg;
    fn on_message(&mut self, _: StateMsg) -> StateMsg {
        StateMsg::None
    }

    fn resize(&mut self, rect: Rect) {
        self.text.position_mut().x = rect.x;
        self.text.position_mut().y = rect.y;
    }

    fn render(&self, renderer: &mut ComponentRenderer) {
        renderer.text(&self.text);
    }

    fn style(&self) -> impl Style + 'static {
        Blue
    }
}

impl CounterText {
    pub fn new(font: CandyFont) -> Self {
        let mut text = CandyText::new("0", Vector2::zeros(), font);

        let mut s = Self { text };
        s.apply_style();
        s
    }
    pub fn apply_style(&mut self) {
        let style = self.style();
        self.text.apply_style(&style);
    }
}

pub enum StateMsg {
    None,
    Increase,
}

#[derive(Default)]
pub struct CounterBtn {
    square: CandySquare,
}

impl CounterBtn {
    pub fn new() -> Self {
        let mut s = Self {
            square: CandySquare::default(),
        };
        s.square.apply_style(&s.style());
        s
    }
}

impl Component for CounterBtn {
    type Message = StateMsg;
    fn on_message(&mut self, _: StateMsg) -> StateMsg {
        StateMsg::None
    }

    fn resize(&mut self, rect: Rect) {
        self.square.position_mut().x = rect.x;
        self.square.position_mut().y = rect.y;
        *self.square.size_mut() = Vector2::new(rect.width, rect.height);
    }

    fn render(&self, renderer: &mut ComponentRenderer) {
        renderer.square(&self.square);
    }

    fn style(&self) -> impl Style + 'static {
        Blue
    }
}

#[derive(Default)]
pub struct State {
    text: CounterText,
    btn: CounterBtn,
    amount: usize,
}

impl Component for State {
    type Message = StateMsg;
    fn resize(&mut self, rect: Rect) {
        let layout = Layout::vertical()
            .with_definition(DefinitionRect {
                x: Size::Percent(0.5),
                y: Size::Percent(0.25),
                width: Size::Length(128.0),
                height: Size::Length(128.0),
            })
            .with_definition(DefinitionRect {
                x: Size::Percent(0.5),
                y: Size::Percent(0.25),
                width: Size::Length(96.0),
                height: Size::Length(48.0),
            });
        let vals = layout.calculate(rect);
        self.text.resize(vals[0].clone());
        self.btn.resize(vals[1].clone());
    }
    fn render(&self, renderer: &mut ComponentRenderer) {
        renderer.background(&Vector4::new(0.0, 0.0, 0.0, 0.3));
        renderer.text(&self.text.text);
        renderer.square(&self.btn.square);
    }
    fn on_message(&mut self, msg: StateMsg) -> StateMsg {
        match msg {
            StateMsg::Increase => {
                self.amount += 1;
                *self.text.text.content_mut() = self.amount.to_string();
            }
            _ => {}
        }
        StateMsg::None
    }
}
use candy::text::manager::FontManager;
impl RootComponent for State {
    fn new() -> Self {
        let manager = FontManager::new();
        Self {
            amount: 0,
            text: CounterText::new(manager.create_font("Fira Sans", 48.0)),
            btn: CounterBtn::new(),
        }
    }
    fn click(&mut self, pos: Vector2<f32>, button: candy::winit::event::MouseButton) -> bool {
        if self.btn.square.bounds().contains(pos) {
            self.on_message(StateMsg::Increase);
        }
        true
    }
}

fn main() {
    CandyWindow::<State>::new(
        WindowAttributes::default()
            .with_transparent(true)
            .with_blur(true),
    )
    .run()
}
