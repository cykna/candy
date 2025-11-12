#[cfg(feature = "opengl")]
use glutin::config::Config;
use glutin::{
    context::PossiblyCurrentContext,
    surface::{Surface, WindowSurface},
};
use nalgebra::{Vector2, Vector4};
use skia_safe::{Data, gpu::gl::FramebufferInfo};
use winit::window::Window;

pub mod candy2d;
pub mod helpers;
pub use candy2d::Candy2DRenderer;

use crate::elements::{image::CandyImage, square::CandySquare, text::CandyText};

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
pub trait BiDimensionalRendererConstructor {
    #[cfg(feature = "opengl")]
    fn new(window: &Window, config: &Config) -> Self;
}
