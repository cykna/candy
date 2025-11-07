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
pub struct Candy2DRenderer {
    environment: Renderer2DEnvironment,
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

impl BiDimensionalRendererConstructor for Candy2DRenderer {
    fn new(window: &Window, config: &Config) -> Self {
        Self {
            environment: Self::create_environment(window, config),
        }
    }
}

impl BiDimensionalRenderer for Candy2DRenderer {
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
    #[cfg(feature = "opengl")]
    fn flush(&mut self) {
        self.environment.gr_context.flush_and_submit();
        self.environment
            .gl_surface
            .swap_buffers(&self.environment.gl_context)
            .unwrap();
    }
    fn painter(&mut self) -> &mut dyn BiDimensionalPainter {
        self
    }
}

impl BiDimensionalPainter for Candy2DRenderer {
    fn square(&mut self, square_info: &CandySquare) {
        let rule = &square_info.rule;

        let radius = rule.border_radius;
        let rect = {
            let position = square_info.position();
            let size = square_info.size();
            Rect::new(
                position.x,
                position.y,
                position.x + size.x,
                position.y + size.y,
            )
        };

        self.canvas()
            .draw_round_rect(rect, radius.x, radius.y, &rule.inner);
        let border_color = rule.border_color;

        if border_color.w == 0.0 || rule.border_width == 0.0 {
            return;
        }
        let mut paint = Paint::new(vec4f32_to_color(&border_color), None);
        paint
            .set_style(skia_safe::PaintStyle::Stroke)
            .set_stroke_width(rule.border_width);

        self.canvas()
            .draw_round_rect(&rect, radius.x, radius.y, &paint);
    }

    #[inline]
    fn circle(&mut self, position: &nalgebra::Vector2<f32>, color: &Vector4<f32>, radius: f32) {
        let paint = Paint::new(vec4f32_to_color(color), None);
        self.canvas()
            .draw_circle(Point::new(position.x, position.y), radius, &paint);
    }

    fn text_sliced(&mut self, info: &CandyText, range: Range<usize>) {
        let rule = &info.rule;

        let bounds = info.bounds();
        let canvas = self.canvas();
        canvas.save();
        canvas.clip_rect(
            Rect {
                left: bounds.x - info.font().size(),
                top: bounds.y - info.font().size(),
                right: bounds.x + bounds.width,
                bottom: bounds.y + bounds.height,
            },
            None,
            Some(true),
        );
        canvas.draw_str(
            &info.content()[range],
            Point::new(info.position().x, info.position().y),
            &info.font(),
            &rule.inner,
        );
        canvas.restore();
    }

    fn text(&mut self, info: &CandyText) {
        let rule = &info.rule;
        let canvas = self.canvas();
        canvas.save();
        let bounds = info.bounds();
        canvas.clip_rect(
            Rect {
                left: bounds.x - info.font().size(),
                top: bounds.y - info.font().size(),
                right: bounds.x + bounds.width,
                bottom: bounds.y + bounds.height,
            },
            None,
            Some(true),
        );
        canvas.draw_str(
            info.content(),
            Point::new(info.position().x, info.position().y),
            &info.font(),
            &rule.inner,
        );
        canvas.restore();
    }
    fn render_image(&mut self, image: &CandyImage) {
        let rule = &image.rule;
        let w = image.real_width();
        let h = image.real_height();
        let position = image.position();
        let rect = Rect::new(
            position.x,
            position.y,
            position.x + w as f32,
            position.y + h as f32,
        );

        let canvas = self.canvas();

        canvas.save();

        canvas.clip_rrect(
            &RRect::new_rect_xy(&rect, rule.border_radius.x, rule.border_radius.y),
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
            &rule.inner,
        );

        canvas.restore();
    }
    fn background(&mut self, color: &Vector4<f32>) {
        self.canvas().clear(*vec4f32_to_color(color));
    }
}
