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
    styling::{self, layout::Size},
};
use window::CandyWindow;
use winit::{event::MouseButton, window::Window};

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

impl Square {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            info: CandySquare::new(
                Vector2::zeros(),
                Vector2::zeros(),
                Vector4::new(r, g, b, 1.0),
                None,
                None,
            ),
        }
    }
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
    w: f32,
    h: f32,
    data: f32,
    squares: Vec<Square>,
}
pub struct Hsv {
    /// Hue in [0,1). 0 and 1 represent the same angle.
    pub h: f32,
    /// Saturation in [0,1]
    pub s: f32,
    /// Value in [0,1]
    pub v: f32,
}

#[inline(always)]
fn clamp01(x: f32) -> f32 {
    x.max(0.0).min(1.0)
}

/// Convert RGB (0..1) -> HSV (h in [0,1))
#[inline]
pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> Hsv {
    // max/min with total order to keep NaNs out of your hot path
    let (maxc, maxi) = {
        // Track which channel was max to compute hue without more branches
        let mut maxv = r;
        let mut idx = 0u8;
        if g > maxv {
            maxv = g;
            idx = 1;
        }
        if b > maxv {
            maxv = b;
            idx = 2;
        }
        (maxv, idx)
    };
    let minc = r.min(g).min(b);
    let delta = maxc - minc;

    // Value is max channel
    let v = maxc;

    // Saturation (guard zero to avoid division)
    let s = if maxc > 0.0 { delta / maxc } else { 0.0 };

    // Hue (normalized to [0,1))
    let mut h = if delta <= 0.0 {
        0.0
    } else {
        // Compute sector offset depending on which channel was the max.
        // These are the standard formulae but we avoid nested branches by
        // keeping only the final add and a single match.
        let base = match maxi {
            0 => (g - b) / delta,       // R is max
            1 => (b - r) / delta + 2.0, // G is max
            _ => (r - g) / delta + 4.0, // B is max
        };
        // Normalize: base in [0,6) -> divide by 6 and wrap
        let h = base / 6.0;
        // frac without calling rem_euclid on floats (faster on many targets)
        h - h.floor()
    };

    // Defensive clamp (cheap) to contain tiny FP slop
    h = if h >= 1.0 {
        0.0
    } else if h < 0.0 {
        h + 1.0
    } else {
        h
    };
    Hsv {
        h,
        s: clamp01(s),
        v: clamp01(v),
    }
}

/// Convert HSV (h in [0,1), s,v in [0,1]) -> RGB (0..1)
/// This is a branch-lite formulation popularized by Iñigo Quílez.
/// It vectorizes well and avoids piecewise "sector" logic.
#[inline]
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    // k = (0, 2/3, 1/3)
    let kx = 0.0f32;
    let ky = 2.0 / 3.0;
    let kz = 1.0 / 3.0;

    // For each channel, compute t = abs(fract(h + k) * 6 - 3) clamped to [0,1]
    #[inline(always)]
    fn chan(h: f32, k: f32) -> f32 {
        let t = (h + k).fract() * 6.0 - 3.0;
        let t = t.abs();
        clamp01(t - 1.0) // equivalently clamp01(1 - |t-2|) but this is fewer ops
    }

    let r = v * (1.0 - s * chan(h, kx));
    let g = v * (1.0 - s * chan(h, ky));
    let b = v * (1.0 - s * chan(h, kz));
    (r, g, b)
}

/// Convenience: u8 <-> float without branching.
/// 0..=255 <-> 0.0..=1.0
#[inline(always)]
pub fn u8_to_f(x: u8) -> f32 {
    (x as f32) * (1.0 / 255.0)
}
#[inline(always)]
pub fn f_to_u8(x: f32) -> u8 {
    (clamp01(x) * 255.0 + 0.5) as u8
}

/// Fast path for 8-bit pixels:
#[inline]
pub fn rgb8_to_hsv(r: u8, g: u8, b: u8) -> Hsv {
    rgb_to_hsv(u8_to_f(r), u8_to_f(g), u8_to_f(b))
}
#[inline]
pub fn hsv_to_rgb8(hsv: Hsv) -> (u8, u8, u8) {
    let (r, g, b) = hsv_to_rgb(hsv.h, hsv.s, hsv.v);
    (f_to_u8(r), f_to_u8(g), f_to_u8(b))
}

impl State {
    fn resize_children(&mut self) {
        let mut style = styling::layout::Layout::default()
            .with_corner(styling::layout::Corner::TopLeft)
            .with_direction(styling::layout::Direction::Horizontal)
            .with_gap(Vector2::new(Size::Length(10.0), Size::Length(10.0)))
            .with_padding(Vector4::new(
                Size::Length(10.0),
                Size::Length(50.0),
                Size::Length(10.0),
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

impl RootComponent for State {
    type Message = Msg;
    fn click(&mut self, position: Vector2<f32>, button: MouseButton) -> bool {
        self.data += 0.1;
        let hsv = hsv_to_rgb(self.data, 1.0, 1.0);
        self.squares.push(Square::new(hsv.0, hsv.1, hsv.2));
        self.resize_children();
        true
    }
    fn resize(&mut self, width: f32, height: f32) {
        self.w = width;
        self.h = height;
        self.resize_children();
    }
    fn render(&self, renderer: &mut ComponentRenderer) {
        renderer.square(&CandySquare::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(self.w, self.h),
            Vector4::new(0.0, 0.0, 0.0, 0.0),
            None,
            None,
        ));
        for s in &self.squares {
            s.render(renderer);
        }
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
