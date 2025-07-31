use raw_window_handle::RawWindowHandle;
use skia_safe::gpu::gl::FramebufferInfo;
use winit::window::Window;

#[cfg(feature = "opengl")]
use glutin::{config::Config, context::NotCurrentContext};

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
