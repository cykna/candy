#[cfg(feature = "opengl")]
mod default_renderer_gl;
#[cfg(feature = "opengl")]
pub use default_renderer_gl::*;

#[cfg(feature = "vulkan")]
mod default_renderer_vk;
#[cfg(feature = "vulkan")]
pub use default_renderer_vk::*;

mod default_painter;

use std::ops::Range;
#[cfg(feature = "opengl")]
use std::sync::Arc;

use nalgebra::{Vector2, Vector4};
#[cfg(feature = "opengl")]
use winit::window::Window;

use crate::primitives::{CandyImage, CandySquare, CandyText};
///Trait used to control a 2D painter

pub trait BiDimensionalRenderer {
    ///When this renderer is requested to resize with the given `width` and `height`
    #[cfg(feature = "opengl")]
    fn resize(&mut self, window: &Window, width: u32, height: u32);

    ///Finishes every command made supposing everything is ready to be drawn on the next frame
    fn flush(&mut self);

    fn painter(&mut self) -> &mut dyn BiDimensionalPainter;
}

///Trait used to control a 2D painter
pub trait BiDimensionalRendererConstructor {
    #[cfg(feature = "opengl")]
    fn new(window: Arc<Window>, config: &glutin::config::Config) -> Self;
}

///A 2D painter used to draw 2D stuff on the screen
pub trait BiDimensionalPainter: BiDimensionalRenderer + std::fmt::Debug {
    ///Method used to draw a square on the screen using the underlying renderer
    fn square(&mut self, square_info: &CandySquare);
    ///Method used to draw a circle on the screen using the underlying renderer
    fn circle(&mut self, position: &Vector2<f32>, color: &Vector4<f32>, radius: f32);

    fn text_sliced(&mut self, info: &CandyText, range: Range<usize>);

    ///Method used to draw a text on the screen using the underlying renderer
    fn text(&mut self, info: &CandyText);

    ///Method uses to draw the given `img` at the given `position`
    fn render_image(&mut self, info: &CandyImage);

    fn background(&mut self, rule: &Vector4<f32>);
}
