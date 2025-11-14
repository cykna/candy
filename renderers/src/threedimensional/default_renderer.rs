use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use candy_shared_types::threed::Mesh;
use vello::wgpu::{
    self, Adapter, BackendOptions, Backends, Color, CommandEncoder, Device, DeviceDescriptor,
    Features, Instance, InstanceFlags, Limits, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
    Trace,
    wgt::{CommandEncoderDescriptor, TextureViewDescriptor},
};

use winit::window::Window;

use crate::{ThreeDimensionalRenderer, ThreeDimensionalRendererConstructor};

#[derive(Debug)]
///The inner state of the default renderer used mainly to talk with wgpu
pub struct WgpuState {
    pub(crate) surface: Surface<'static>,
    adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl WgpuState {
    ///Generates a configuration for the provided `surface` with the capabilities ofthe provided `adapter`
    fn surface_config(
        surface: &Surface,
        adapter: &Adapter,
        width: u32,
        height: u32,
    ) -> SurfaceConfiguration {
        let capabilities = surface.get_capabilities(adapter);
        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            format: wgpu::TextureFormat::Rgba8Unorm,
            width,
            height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        }
    }

    ///Retrieves a new State to work with the provided `window`. Panics if something goes wrong during initialization, since this is a basic operation to the software start operating
    fn new(window: Arc<Window>) -> (Self, SurfaceConfiguration) {
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: if cfg!(feature = "vulkan") {
                Backends::VULKAN
            } else {
                Backends::all()
            },
            flags: InstanceFlags::all(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: BackendOptions::default(),
        });
        let window_size = window.inner_size();
        let surface = instance.create_surface(window).unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
            label: Some("Device creation"),
            required_features: Features::empty(),
            required_limits: Limits::defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: Trace::Off,
        }))
        .unwrap();

        let surface_config =
            Self::surface_config(&surface, &adapter, window_size.width, window_size.height);
        surface.configure(&device, &surface_config);
        (
            Self {
                surface,
                adapter,
                device,
                queue,
            },
            surface_config,
        )
    }
}

///The default 3D renderer used by Candy. Note that on it's creation, on failures, it will panic.
pub struct Candy3DefaultRenderer {
    state: Arc<WgpuState>,
    config: SurfaceConfiguration,
}

impl ThreeDimensionalRenderer for Candy3DefaultRenderer {
    fn state(&self) -> Arc<WgpuState> {
        self.state.clone()
    }
    fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.state
            .surface
            .configure(&self.state.device, &self.config);
    }
    ///Returns the surface texture to be able to draw other things after it
    fn render<'a>(
        &mut self,
        scene: Option<impl Iterator<Item = &'a Mesh>>,
    ) -> (wgpu::SurfaceTexture, wgpu::TextureView, CommandEncoder) {
        let texture = self.surface.get_current_texture().unwrap();

        let view = texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Encoder descriptor"),
            });
        {
            let pass = encoder.begin_render_pass(&RenderPassDescriptor {
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                label: Some("Render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: wgpu::LoadOp::Clear(Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
            });
        };
        (texture, view, encoder)
    }
}
impl ThreeDimensionalRendererConstructor for Candy3DefaultRenderer {
    fn new(window: Arc<Window>) -> Self {
        let (state, config) = WgpuState::new(window);
        Self {
            state: Arc::new(state),
            config,
        }
    }
}

impl Deref for Candy3DefaultRenderer {
    type Target = Arc<WgpuState>;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Candy3DefaultRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
