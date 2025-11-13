use std::sync::Arc;

use nalgebra::Vector4;
use vello::{
    AaSupport, RenderParams, Renderer, RendererOptions, Scene,
    kurbo::{self, Affine, Rect},
    peniko::{BlendMode, BrushRef, Fill, color::AlphaColor},
    wgpu::{
        self, BlendComponent, BlendFactor, BlendOperation, BlendState, Color, CommandEncoder,
        CommandEncoderDescriptor, Extent3d, Operations, RenderPassColorAttachment,
        RenderPassDescriptor, Texture, TextureUsages, TextureView, TextureViewDescriptor,
        util::{TextureBlitter, TextureBlitterBuilder},
        wgt::TextureDescriptor,
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
    view: TextureView,
    blitter: TextureBlitter,
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
        let blitter = TextureBlitterBuilder::new(&state.device, texture.format())
            .sample_type(wgpu::FilterMode::Linear)
            .blend_state(BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
            })
            .build();
        Self {
            view: texture.create_view(&TextureViewDescriptor {
                label: None,
                ..Default::default()
            }),
            blitter,
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
        let view = self.texture.create_view(&TextureViewDescriptor::default());
        let mut clear_encoder = self
            .state
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let _pass = clear_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("clear vello texture"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: Operations {
                        load: wgpu::LoadOp::Clear(Color {
                            r: self.background.x as f64,
                            g: self.background.y as f64,
                            b: self.background.z as f64,
                            a: self.background.w as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }

        self.state.queue.submit(Some(clear_encoder.finish()));
        self.renderer
            .render_to_texture(
                &self.state.device,
                &self.state.queue,
                &self.scene,
                &view,
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

        self.blitter
            .copy(&self.state.device, encoder, &view, texture);
        self.scene.reset();
    }
    fn painter(&mut self) -> &mut dyn super::BiDimensionalPainter {
        self
    }
    fn prepare(&mut self) {}
}

impl BiDimensionalPainter for Candy2DefaultRenderer {
    fn square(&mut self, square_info: &crate::primitives::CandySquare) {
        let rule = &square_info.rule;

        let radius = rule.border_radius;
        let rect = {
            let position = square_info.position();
            let size = square_info.size();
            kurbo::Rect {
                x0: position.x as f64,
                y0: position.y as f64,
                x1: (position.x + size.x) as f64,
                y1: (position.y + size.y) as f64,
            }
        };

        let color = rule.get_color();

        let window = self.window.inner_size();

        self.scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            BrushRef::Solid(AlphaColor::new([color.x, color.y, color.z, color.w])),
            None,
            &rect,
        );

        let border_color = rule.border_color;

        if border_color.w == 0.0 || rule.border_width == 0.0 {
            return;
        }
    }
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
