use std::ops::Range;

use candy_shared_types::vec4f32_to_color;
#[cfg(feature = "opengl")]
use glutin::{
    config::Config,
    context::{NotCurrentContext, PossiblyCurrentContext},
    surface::{Surface, WindowSurface},
};
use nalgebra::Vector4;
#[cfg(feature = "opengl")]
use raw_window_handle::RawWindowHandle;
#[cfg(feature = "opengl")]
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::{Canvas, Paint, Point, RRect, Rect, SamplingOptions, canvas::SrcRectConstraint};
#[cfg(feature = "opengl")]
use winit::window::Window;

use crate::{
    BiDimensionalPainter, BiDimensionalRenderer, BiDimensionalRendererConstructor,
    primitives::{CandyImage, CandySquare, CandyText},
};

pub fn create_surface(
    window: &Window,
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> skia_safe::Surface {
    use skia_safe::gpu::backend_render_targets;

    let size = window.inner_size();
    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );
    let backend_render_target =
        backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);
    skia_safe::gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        skia_safe::gpu::SurfaceOrigin::BottomLeft,
        skia_safe::ColorType::BGRA8888,
        None,
        None,
    )
    .unwrap()
}
#[cfg(feature = "opengl")]
pub fn create_context(handle: Option<RawWindowHandle>, config: &Config) -> NotCurrentContext {
    use glutin::{
        context::{ContextApi, ContextAttributesBuilder},
        display::GetGlDisplay,
        prelude::GlDisplay,
    };

    let default_attributes = ContextAttributesBuilder::new().build(handle);
    let es_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(handle);
    let display = config.display();
    unsafe {
        display
            .create_context(config, &default_attributes)
            .unwrap_or_else(|_| {
                display
                    .create_context(config, &es_attributes)
                    .expect("Could not create opengl context. Device not supported")
            })
    }
}

#[derive(Debug)]
///Internal environemtn for rendering stuff on the 2D default renderer
struct Renderer2DEnvironment {
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

#[derive(Debug)]
///The default renderer that is be used when drawing 2D
pub struct Candy2DefaultRenderer {
    environment: Renderer2DEnvironment,
}
impl BiDimensionalRendererConstructor for Candy2DefaultRenderer {
    fn new(window: &Window, config: &Config) -> Self {
        Self {
            environment: Self::create_environment(window, config),
        }
    }
}
impl Candy2DefaultRenderer {
    #[cfg(feature = "opengl")]
    fn create_environment(window: &Window, config: &Config) -> Renderer2DEnvironment {
        use std::ffi::CString;
        use std::num::NonZero;

        use glutin::config::GlConfig;
        use glutin::{display::GetGlDisplay, prelude::GlDisplay};
        use glutin::{prelude::NotCurrentGlContext, surface::SurfaceAttributesBuilder};
        use raw_window_handle::HasWindowHandle;
        use winit::dpi::PhysicalSize;

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
    ///Retrieves the canvas of this renderer
    fn canvas(&mut self) -> &Canvas {
        self.environment.surface.canvas()
    }
}

impl BiDimensionalRenderer for Candy2DefaultRenderer {
    #[cfg(feature = "opengl")]
    fn resize(&mut self, window: &Window, width: u32, height: u32) {
        use std::num::NonZero;

        use glutin::surface::GlSurface;

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
        use glutin::surface::GlSurface;

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

impl BiDimensionalPainter for Candy2DefaultRenderer {
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
