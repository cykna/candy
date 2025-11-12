use glutin::{
    config::{Config, GlConfig},
    context::NotCurrentGlContext,
    display::{GetGlDisplay, GlDisplay},
    prelude::GlSurface,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use nalgebra::Vector4;
use raw_window_handle::HasWindowHandle;
use skia_safe::{
    Canvas, Paint, Point, RRect, Rect, SamplingOptions, canvas::SrcRectConstraint,
    gpu::gl::FramebufferInfo,
};
use std::{ffi::CString, num::NonZero, ops::Range};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    elements::{image::CandyImage, square::CandySquare, text::CandyText},
    helpers::vec4f32_to_color,
    renderer::twod::BiDimensionalRendererConstructor,
};

use super::{
    BiDimensionalPainter, BiDimensionalRenderer, Renderer2DEnvironment,
    helpers::{create_context, create_surface},
};

#[derive(Debug)]
///Default 2D renderer of Candy. By default a wrapper over skia-safe

impl Candy2DRenderer {
    pub fn canvas(&mut self) -> &Canvas {
        self.environment.surface.canvas()
    }
}

impl BiDimensionalRendererConstructor for Candy2DRenderer {
    fn new(window: &Window, config: &Config) -> Self {
        Self {
            environment: Self::create_environment(window, config),
        }
    }
}
