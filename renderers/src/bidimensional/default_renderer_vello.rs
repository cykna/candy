use std::sync::Arc;

use nalgebra::Vector4;
use vello::{
    AaSupport, RenderParams, Renderer, RendererOptions, Scene,
    peniko::color::AlphaColor,
    wgpu::{
        self, CommandEncoder, CommandEncoderDescriptor, Extent3d, Texture, TextureUsages,
        TextureView, TextureViewDescriptor, wgt::TextureDescriptor,
    },
};
use winit::window::Window;

use crate::{
    BiDimensionalPainter, BiDimensionalRenderer, BiDimensionalRendererConstructor, WgpuState,
};

///The default renderer that is be used when drawing 2D
pub struct Candy2DefaultRenderer {
    renderer: Renderer,
    scene: Scene,
    state: Arc<WgpuState>,
    window: Arc<Window>,
    background: Vector4<f32>,
    texture: Texture,
}
impl BiDimensionalRendererConstructor for Candy2DefaultRenderer {
    fn new(window: Arc<Window>, state: Arc<WgpuState>) -> Self {
        let renderer = vello::Renderer::new(
            &state.device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: AaSupport::area_only(),
                num_init_threads: None,
                pipeline_cache: None,
            },
        )
        .unwrap();
        let size = window.inner_size();
        let texture = state.device.create_texture(&TextureDescriptor {
            label: Some("2d image"),
            size: Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            dimension: wgpu::TextureDimension::D2,
            mip_level_count: 1,
            sample_count: 1,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC
                | TextureUsages::STORAGE_BINDING
                | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        Self {
            window,
            renderer,
            scene: Scene::new(),
            state,
            background: Vector4::zeros(),
            texture,
        }
    }
}

impl BiDimensionalRenderer for Candy2DefaultRenderer {
    fn flush(&mut self, texture: &TextureView, encoder: &mut CommandEncoder) {
        let src = self.texture.create_view(&TextureViewDescriptor {
            label: None,
            ..Default::default()
        });
        self.renderer
            .render_to_texture(
                &self.state.device,
                &self.state.queue,
                &self.scene,
                &src,
                &RenderParams {
                    base_color: AlphaColor {
                        components: [
                            self.background.x,
                            self.background.y,
                            self.background.z,
                            self.background.w,
                        ],
                        cs: std::marker::PhantomData,
                    },
                    width: self.window.inner_size().width,
                    height: self.window.inner_size().height,
                    antialiasing_method: vello::AaConfig::Area,
                },
            )
            .unwrap();
        let blitter = wgpu::util::TextureBlitter::new(&self.state.device, self.texture.format());
        blitter.copy(&self.state.device, encoder, &src, texture);
    }
    fn painter(&mut self) -> &mut dyn super::BiDimensionalPainter {
        self
    }
}

impl BiDimensionalPainter for Candy2DefaultRenderer {
    fn square(&mut self, square_info: &crate::primitives::CandySquare) {}
    fn circle(
        &mut self,
        position: &nalgebra::Vector2<f32>,
        color: &nalgebra::Vector4<f32>,
        radius: f32,
    ) {
    }
    fn text_sliced(&mut self, info: &crate::primitives::CandyText, range: std::ops::Range<usize>) {}
    fn text(&mut self, info: &crate::primitives::CandyText) {}
    fn render_image(&mut self, info: &crate::primitives::CandyImage) {}
    fn background(&mut self, background: &nalgebra::Vector4<f32>) {
        self.background = background.clone_owned();
    }
}

impl std::fmt::Debug for Candy2DefaultRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Candy2DefaultRenderer")
            .field("environment", &"internal")
            .field("scene", &"internal")
            .field("wgpu_state", &self.state)
            .finish()
    }
}
