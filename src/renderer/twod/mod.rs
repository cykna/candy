use std::ops::Range;

use glutin::{
    config::Config,
    context::PossiblyCurrentContext,
    surface::{Surface, WindowSurface},
};
use nalgebra::{Vector2, Vector4};
use skia_safe::{Data, gpu::gl::FramebufferInfo};
use winit::window::Window;

pub mod candy2d;
pub mod helpers;
pub use candy2d::Candy2DRenderer;

use crate::{
    elements::{
        image::{CandyImage, TwodCandyImg},
        square::CandySquare,
        text::CandyText,
    },
    renderer::CandyRenderer,
};

#[derive(Debug)]
pub struct Renderer2DEnvironment {
    surface: skia_safe::Surface,
    #[cfg(feature = "opengl")]
    gl_surface: Surface<WindowSurface>,
    #[cfg(feature = "opengl")]
    gr_context: skia_safe::gpu::DirectContext,
    #[cfg(feature = "opengl")]
    gl_context: PossiblyCurrentContext,
    #[cfg(feature = "opengl")]
    fb_info: FramebufferInfo,
    #[cfg(feature = "opengl")]
    samples: usize,
    #[cfg(feature = "opengl")]
    stencil_size: usize,
}
///Trait used to control a 2D painter
pub trait BiDimensionalRenderer: BiDimensionalPainter {
    #[cfg(feature = "opengl")]
    fn new(window: &Window, config: &Config) -> Self;

    ///When this renderer is requested to resize with the given `width` and `height`
    #[cfg(feature = "opengl")]
    fn resize(&mut self, window: &Window, width: u32, height: u32);

    ///Retrieves the struct that actually does draw things on the screen
    fn twod_painter(&mut self) -> &mut impl BiDimensionalPainter;

    ///Finishes every command made supposing everything is ready to be drawn on the next frame
    fn flush(&mut self);
}

///A 2D painter used to draw 2D stuff on the screen
pub trait BiDimensionalPainter: Sized + std::fmt::Debug {
    type Image: TwodCandyImg;
    ///Method used to draw a square on the screen using the underlying renderer
    fn square(&mut self, square_info: &CandySquare);
    ///Method used to draw a circle on the screen using the underlying renderer
    fn circle(&mut self, position: &Vector2<f32>, color: &Vector4<f32>, radius: f32);

    fn text_sliced(&mut self, info: &CandyText, range: Range<usize>);

    ///Method used to draw a text on the screen using the underlying renderer
    fn text(&mut self, info: &CandyText);

    ///Method uses to draw the given `img` at the given `position`
    fn render_image(&mut self, info: &CandyImage<Self>);

    fn background(&mut self, rule: &Vector4<f32>);
}

pub struct RenderImageOptions {
    pub border_radius: Vector2<f32>,
    pub border_color: Vector4<f32>,
    pub border_width: f32,
}

pub trait CandyImgConstructor<I> {
    fn from_bytes(bytes: &[u8]) -> I;
}

impl CandyImgConstructor<skia_safe::Image> for skia_safe::Image {
    fn from_bytes(bytes: &[u8]) -> skia_safe::Image {
        skia_safe::Image::from_encoded(Data::new_copy(bytes)).unwrap()
    }
}
