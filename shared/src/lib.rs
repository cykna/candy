//This crate contains only the abstractions over `renderer` and the most higher level features used on the library
mod rect;
pub mod threed;
mod ui;
use nalgebra::Vector4;
pub use rect::*;
use skia_safe::Color4f;
pub use ui::*;

///Casts the given vector into a Color4f.
pub fn vec4f32_to_color(color: &Vector4<f32>) -> &Color4f {
    unsafe { std::mem::transmute::<&Vector4<f32>, &Color4f>(color) }
}

///Casts the given vector into a Color4f.
pub fn vec4f32_to_color_value(color: Vector4<f32>) -> Color4f {
    unsafe { std::mem::transmute::<Vector4<f32>, Color4f>(color) }
}

///Casts the given `color` into a `rect`(???)
pub fn vec4f32_to_rect(color: &Vector4<f32>) -> &Rect {
    unsafe { std::mem::transmute::<&Vector4<f32>, &Rect>(color) }
}

#[cfg(feature = "skia")]
pub type RendererImage = skia_safe::Image;
#[cfg(feature = "vello")]
pub type RendererImage = ();
