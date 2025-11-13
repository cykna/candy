use std::{ops::Range, sync::Arc};

use candy_shared_types::vec4f32_to_color;
#[cfg(feature = "opengl")]
use glutin::{
    config::Config,
    context::{NotCurrentContext, PossiblyCurrentContext},
    surface::{Surface, WindowSurface},
};

#[cfg(feature = "opengl")]
use raw_window_handle::RawWindowHandle;

#[cfg(feature = "opengl")]
use skia_safe::gpu::gl::FramebufferInfo;

use nalgebra::Vector4;
use skia_safe::{
    Canvas, Paint, Point, RRect, Rect, SamplingOptions,
    canvas::SrcRectConstraint,
    gpu::{ContextOptions, DirectContext, gl::Interface},
};

use winit::window::Window;

use crate::{
    BiDimensionalPainter, BiDimensionalRenderer, BiDimensionalRendererConstructor,
    primitives::{CandyImage, CandySquare, CandyText},
};

#[cfg(feature = "opengl")]
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

#[cfg(open)]
pub fn create_skia_surface(
    window: &Window,
    fb: FramebufferInfo,
    samples: usize,
    stencil_size: usize,
    interface: Interface,
) -> (skia_safe::Surface, DirectContext) {
    let mut ctx = skia_safe::gpu::direct_contexts::make_gl(interface, &ContextOptions::default())
        .expect("Could not create direct context for opengl");

    let size = window.inner_size();
    let render_target = skia_safe::gpu::backend_render_targets::make_gl(
        (size.width as i32, size.height as i32),
        samples,
        stencil_size,
        fb,
    );
    (
        skia_safe::gpu::surfaces::wrap_backend_render_target(
            &mut ctx,
            &render_target,
            skia_safe::gpu::SurfaceOrigin::TopLeft,
            skia_safe::ColorType::RGBA8888,
            None,
            None,
        )
        .unwrap(),
        ctx,
    )
}

#[derive(Debug)]
#[cfg(feature = "opengl")]
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
    fn new(window: Arc<Window>, config: &Config) -> Self {
        Self {
            environment: Self::create_environment(&window, config),
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
        let (surface, gr_context) =
            create_skia_surface(&window, fb, samples, stencil_size, interface);

        Renderer2DEnvironment {
            surface,
            gl_context,
            gr_context,
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
