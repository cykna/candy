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
    Canvas, Color4f, Paint, Point, RRect, Rect, SamplingOptions,
    canvas::SrcRectConstraint,
    gpu::gl::FramebufferInfo,
    image_filters::{self, CropRect},
};
use std::{ffi::CString, num::NonZero};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    elements::{image::CandyImage, square::CandySquare, text::CandyText},
    helpers::vec4f32_to_color,
};

use super::{
    BiDimensionalPainter, BiDimensionalRenderer, Renderer2DEnvironment,
    helpers::{create_context, create_surface},
};

#[derive(Debug)]
///Default 2D renderer of Candy. By default a wrapper over skia-safe
pub struct Candy2DRenderer {
    environment: Renderer2DEnvironment,
    paint: Paint,
}

impl Candy2DRenderer {
    #[cfg(feature = "opengl")]
    pub fn create_environment(window: &Window, config: &Config) -> Renderer2DEnvironment {
        let handle = window
            .window_handle()
            .ok()
            .map(|window_handle| window_handle.as_raw());

        let PhysicalSize { width, height } = window.inner_size();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            handle.unwrap(),
            NonZero::new(width).unwrap(),
            NonZero::new(height).unwrap(),
        );
        let gl_surface = unsafe {
            config
                .display()
                .create_window_surface(&config, &attrs)
                .unwrap()
        };
        let context = create_context(handle, config);
        let gl_context = context.make_current(&gl_surface).unwrap();
        gl::load_with(|s| {
            config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });
        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                std::ptr::null()
            } else {
                config
                    .display()
                    .get_proc_address(CString::new(name).unwrap().as_c_str())
            }
        })
        .unwrap();
        let mut gr_context = skia_safe::gpu::direct_contexts::make_gl(interface, None).unwrap();

        let fb = {
            let mut fbid = 0;
            unsafe {
                gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fbid);
            }
            FramebufferInfo {
                fboid: fbid as u32,
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };
        let samples = config.num_samples() as usize;
        let stencil_size = config.stencil_size() as usize;
        let surface = create_surface(window, fb, &mut gr_context, samples, stencil_size);
        Renderer2DEnvironment {
            surface,
            gr_context,
            gl_context,
            gl_surface,
            fb_info: fb,
            samples,
            stencil_size,
        }
    }
}

impl Candy2DRenderer {
    pub fn canvas(&mut self) -> &Canvas {
        self.environment.surface.canvas()
    }
}

impl BiDimensionalRenderer for Candy2DRenderer {
    #[inline]
    fn new(window: &Window, config: &Config) -> Self {
        Self {
            environment: Self::create_environment(window, config),
            paint: Paint::new(Color4f::new(0.0, 0.0, 0.0, 0.0), None),
        }
    }

    #[cfg(feature = "opengl")]
    fn resize(&mut self, window: &Window, width: u32, height: u32) {
        self.environment.surface = create_surface(
            window,
            self.environment.fb_info,
            &mut self.environment.gr_context,
            self.environment.samples,
            self.environment.stencil_size,
        );
        self.environment.gl_surface.resize(
            &self.environment.gl_context,
            NonZero::new(width.max(1)).unwrap(),
            NonZero::new(height.max(1)).unwrap(),
        );
    }
    #[inline]
    fn twod_painter(&mut self) -> &mut impl BiDimensionalPainter {
        self
    }
    #[inline]
    #[cfg(feature = "opengl")]
    fn flush(&mut self) {
        self.environment.gr_context.flush_and_submit();
        self.environment
            .gl_surface
            .swap_buffers(&self.environment.gl_context)
            .unwrap();
    }
}

impl Candy2DRenderer {
    ///Creates a new paint with all the configurations needed to draw `info` square properly
    pub(crate) fn create_paint_for_square(&mut self, info: &CandySquare) {
        let color = unsafe { std::mem::transmute::<_, &Color4f>(info.background_color()) };
        self.paint.set_color4f(color, None);

        self.paint.set_style(skia_safe::PaintStyle::Fill);
        let filter = image_filters::drop_shadow(
            Point::new(0.0, 0.0),
            (10.0, 10.0),
            Color4f::new(1.0, 1.0, 0.0, 1.0),
            None,
            None,
            CropRect::NO_CROP_RECT,
        );
        self.paint.set_image_filter(filter);
    }
}

impl BiDimensionalPainter for Candy2DRenderer {
    type Image = skia_safe::Image;
    fn square(&mut self, square_info: &CandySquare) {
        self.create_paint_for_square(square_info);
        let radius = square_info.border_radius();

        let position = square_info.position();
        let size = square_info.size();
        let rect = Rect::new(
            position.x,
            position.y,
            position.x + size.x,
            position.y + size.y,
        );

        let border_color = square_info.border_color();

        let mut paint = self.paint.clone();
        self.canvas()
            .draw_round_rect(&rect, radius.x, radius.y, &paint);

        if border_color.w == 0.0 {
            return;
        };
        paint
            .set_color4f(
                unsafe { std::mem::transmute::<_, &Color4f>(square_info.border_color()) },
                None,
            )
            .set_style(skia_safe::PaintStyle::Stroke)
            .set_stroke_width(2.0);
        self.canvas()
            .draw_round_rect(&rect, radius.x, radius.y, &paint);
    }

    #[inline]
    fn circle(&mut self, position: &nalgebra::Vector2<f32>, color: &Vector4<f32>, radius: f32) {
        let paint = Paint::new(vec4f32_to_color(color), None);
        self.canvas()
            .draw_circle(Point::new(position.x, position.y), radius, &paint);
    }

    fn text(&mut self, info: &CandyText) {
        let mut paint = Paint::new(vec4f32_to_color(info.color()), None);
        paint.set_anti_alias(true);
        self.canvas().draw_str(
            info.content(),
            Point::new(info.position().x, info.position().y),
            &info.font(),
            &paint,
        );
    }
    fn render_image(&mut self, image: &CandyImage<Self>) {
        let w = image.real_width();
        let h = image.real_height();
        let position = image.position();
        let rect = Rect::new(
            position.x,
            position.y,
            position.x + w as f32,
            position.y + h as f32,
        );

        let paint = Paint::default();

        let canvas = self.canvas();

        canvas.save();

        canvas.clip_rrect(
            &RRect::new_rect_xy(&rect, image.border_radius().x, image.border_radius().y),
            None,
            true,
        );

        canvas.draw_image_rect_with_sampling_options(
            image.image_handler(),
            Some((
                &Rect::new(0.0, 0.0, w as f32, h as f32),
                SrcRectConstraint::Fast,
            )),
            rect,
            SamplingOptions::default(),
            &paint,
        );

        canvas.restore();
    }
    fn background(&mut self, color: &Vector4<f32>) {
        self.canvas().clear(*vec4f32_to_color(color));
    }
}
