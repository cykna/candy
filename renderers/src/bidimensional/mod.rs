use std::ops::Range;

use nalgebra::{Vector2, Vector4};
use winit::window::Window;
mod default_renderer;
pub use default_renderer::*;
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
    fn new(window: &Window, config: &glutin::config::Config) -> Self;
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
