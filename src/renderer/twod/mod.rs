use std::any::Any;

use glutin::{
    config::Config,
    context::PossiblyCurrentContext,
    surface::{Surface, WindowSurface},
};
use nalgebra::{Vector2, Vector4};
use skia_safe::{Data, RCHandle, gpu::gl::FramebufferInfo};
use winit::window::Window;

pub mod candy2d;
pub mod helpers;
pub use candy2d::Candy2DRenderer;

use crate::elements::{image::CandyImage, square::CandySquare};

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

pub trait BiDimensionalRenderer {
    #[cfg(feature = "opengl")]
    fn new(window: &Window, config: &Config) -> Self;

    #[cfg(feature = "opengl")]
    fn resize(&mut self, window: &Window, width: u32, height: u32);

    fn twod_painter(&mut self) -> &mut impl BiDimensionalPainter;

    fn flush(&mut self);
}

pub trait BiDimensionalPainter {
    fn square(&mut self, square_info: &CandySquare);
    fn image(&mut self, image_info: &CandyImage);
    fn circle(&mut self, position: &Vector2<f32>, color: &Vector4<f32>, radius: f32);
}

pub trait CandyImgConstructor<I> {
    fn from_bytes(bytes: &[u8]) -> I;
}

/// A handler for Images on Candy. This is now shown due to rust limitations with dyn, but this is dependent of CandyImgConstructor
pub trait CandyImg: Any {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

impl CandyImgConstructor<skia_safe::Image> for skia_safe::Image {
    fn from_bytes(bytes: &[u8]) -> skia_safe::Image {
        skia_safe::Image::from_encoded(Data::new_copy(bytes)).unwrap()
    }
}

impl CandyImg for skia_safe::Image {
    #[inline]
    fn width(&self) -> u32 {
        self.width() as u32
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height() as u32
    }
}
