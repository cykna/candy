pub mod components;

pub mod helpers;
pub mod renderer;

pub mod text;
pub mod ui;
pub mod window;

use std::f32;
use std::sync::Arc;
use std::time::Duration;

use crate::components::{Scrollable, ScrollableConfig};

use crate::text::manager::FontManager;
use crate::ui::animation::manager::AnimationManager;
use crate::ui::animation::scheduler::{AnimationScheduler, SchedulerSender};
use crate::ui::animation::{Animatable, Animation, AnimationConfig, AnimationState};

use crate::ui::styling::layout::Layout;
use crate::ui::styling::layout::{DefinitionRect, Direction};

use crate::ui::animation::curves::LinearCurve;

use candy_renderers::primitives::{CandyFont, CandySquare, CandyText};
use candy_renderers::{BiDimensionalPainter, CandyDefaultRenderer};
use candy_shared_types::{Rect, Style};
use nalgebra::{Vector2, Vector4};

use crate::ui::{
    component::{Component, RootComponent},
    styling::layout::Size,
};
use window::CandyWindow;

use winit::{event::MouseButton, window::Window};

#[cfg(feature = "opengl")]
pub use glutin::config::Config;

pub enum Msg {
    None,
    MarkUndirty,
    Write(String),
}

#[derive(Debug)]
pub struct Square {
    text: CandyText,
    info: CandySquare,
}

impl Square {
    pub fn new(font: CandyFont) -> Self {
        Self {
            text: CandyText::new("pedro", Vector2::zeros(), font),
            info: CandySquare::new(Vector2::zeros(), Vector2::zeros()),
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

    fn render(&self, renderer: &mut dyn BiDimensionalPainter) {
        renderer.square(&self.info);
        renderer.text(&self.text);
    }

    fn apply_style(&mut self, style: &dyn Style) {
        self.info.apply_style(style);
    }
    fn position(&self) -> Vector2<f32> {
        *self.info.position()
    }
    fn position_mut(&mut self) -> &mut Vector2<f32> {
        self.info.position_mut()
    }
}

struct State {
    window: Arc<Window>,
    pos: Vector2<f32>,
    idx: usize,
    w: f32,
    h: f32,
    data: Scrollable<Square>,
    manager: FontManager,
    anims: SchedulerSender,
}

impl Component for State {
    fn resize(&mut self, rect: Rect) {
        self.w = rect.width;
        self.h = rect.height;
        self.data.resize(rect.clone());
    }
    fn render(&self, renderer: &mut dyn BiDimensionalPainter) {
        renderer.background(&Vector4::new(0.0, 0.1, 0.2, 0.5));
        self.data.render(renderer);
    }
    fn apply_style(&mut self, _: &dyn Style) {}
    fn position(&self) -> Vector2<f32> {
        self.pos
    }
    fn position_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.pos
    }
}

#[derive(Debug)]
pub struct AnimState {
    color: Vector4<f32>,
    pos: Vector2<f32>,
}
impl AnimState {
    pub fn black(pos: Vector2<f32>) -> Self {
        Self {
            color: Vector4::new(0.0, 0.0, 0.0, 1.0),
            pos,
        }
    }

    pub fn white(pos: Vector2<f32>) -> Self {
        Self {
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            pos,
        }
    }
}
impl Style for AnimState {
    fn color(&self) -> Vector4<f32> {
        self.color
    }
    fn background_color(&self) -> Vector4<f32> {
        self.color
    }
}
impl AnimationState for AnimState {
    fn lerp(start: &Self, end: &Self, cdt: f32, dt: f32) -> Self {
        let final_pos = { start.pos.lerp(&end.pos, cdt) };
        Self {
            color: start.color.lerp(&end.color, cdt),
            pos: final_pos,
        }
    }
    fn apply_to(&self, comp: &mut dyn crate::ui::component::Component) {
        comp.apply_style(self);
        comp.apply_offset(self.pos / 100.0);
    }
}

impl RootComponent for State {
    type Args = ();
    fn new(window: Arc<Window>, _: ()) -> Self {
        let font = FontManager::new();

        Self {
            window,
            idx: 0,
            anims: {
                let manager = AnimationManager::new();
                manager.start_execution()
            },
            w: 0.0,
            h: 0.0,
            pos: Vector2::zeros(),
            data: {
                let scroll = Scrollable::new(ScrollableConfig {
                    layout: {
                        let mut out = Layout::vertical();
                        out.with_gap(Vector2::new(Size::Length(0.0), Size::Length(10.0)));
                        out
                    },
                    scroll_bar_width: 10.0,
                    direction: Direction::Vertical,
                });
                scroll
            },
            manager: font,
        }
    }
    fn window(&self) -> &Window {
        &self.window
    }

    fn keyup(
        &mut self,
        _: winit::keyboard::Key<winit::keyboard::SmolStr>,
        _: winit::keyboard::KeyLocation,
    ) -> bool {
        false
    }
    fn on_mouse_wheel(
        &mut self,
        offset: winit::event::MouseScrollDelta,
        _: winit::event::TouchPhase,
    ) -> bool {
        match offset {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                self.data.drag_offset(Vector2::new(x, -y))
            }
            _ => false,
        }
    }
    fn on_mouse_move(&mut self, pos: Vector2<f32>) -> bool {
        self.data.drag(pos);

        false
    }
    fn click(&mut self, _: MouseButton) -> bool {
        self.data.on_mouse_click(Vector2::new(0.0, 0.0));

        let font = self.manager.create_font("Nimbus Roman", 24.0).unwrap();
        let mut s = Square::new(font);
        *s.text.content_mut() = format!("Hello {}", self.data.children().len());

        self.data.add_child(
            s,
            DefinitionRect {
                x: Size::Length(0.0),
                y: Size::Length(0.0),
                width: Size::Percent(0.25),
                height: Size::Percent(0.25),
            },
        );
        let mut delay = 0;
        for child in self.data.children_mut() {
            child.play_animation(
                Animation::new::<LinearCurve>(
                    AnimState::black(Vector2::new(0.0, 0.0)),
                    AnimState::white(Vector2::new(100.0, 100.0)),
                    Duration::from_secs(3),
                    Duration::from_millis(16),
                ),
                AnimationConfig {
                    delay: Duration::from_millis(delay),
                },
                self.anims.clone(),
            );
            delay += 250;
        }

        self.resize(Rect {
            x: 0.0,
            y: 0.0,
            width: self.w,
            height: self.h,
        });
        true
    }
    fn check_updates(&mut self) -> bool {
        self.idx += 1;
        true
    }
}

fn main() {
    CandyWindow::<State, CandyDefaultRenderer>::new(
        Window::default_attributes()
            .with_transparent(true)
            .with_title("Candy"),
    )
    .run();
}
