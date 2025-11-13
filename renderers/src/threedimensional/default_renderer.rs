use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
};

use wgpu::{
    Adapter, BackendOptions, Backends, Color, Device, DeviceDescriptor, ExperimentalFeatures,
    Features, Instance, InstanceFlags, Limits, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages, Trace,
    wgt::{CommandEncoderDescriptor, TextureViewDescriptor},
};
use wgpu_hal::{Api, DynDevice};
use winit::window::Window;

use crate::{ThreeDimensionalRenderer, ThreeDimensionalRendererConstructor};

///The inner state of the default renderer used mainly to talk with wgpu
pub struct WgpuState {
    pub(crate) surface: Surface<'static>,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
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
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width,
            height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        }
    }

    ///Retrieves a new State to work with the provided `window`. Panics if something goes wrong during initialization, since this is a basic operation to the software start operating
    fn new(window: Arc<Window>) -> Self {
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: if cfg!(feature = "vulkan") {
                Backends::VULKAN
            } else {
                Backends::GL
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
            experimental_features: ExperimentalFeatures::disabled(),
            trace: Trace::Off,
        }))
        .unwrap();

        let surface_config =
            Self::surface_config(&surface, &adapter, window_size.width, window_size.height);
        surface.configure(&device, &surface_config);
        Self {
            surface,
            adapter,
            device,
            queue,
            config: surface_config,
        }
    }
}

///The default 3D renderer used by Candy. Note that on it's creation, on failures, it will panic.
pub struct Candy3DefaultRenderer {
    state: WgpuState,
}

impl ThreeDimensionalRenderer for Candy3DefaultRenderer {
    fn resize(&mut self, width: u32, height: u32) {
        self.state.config.width = width;
        self.state.config.height = height;
    }
    ///Returns the surface texture to be able to draw other things after it
    fn render(
        &mut self,
        scene: Option<&dyn candy_shared_types::threed::ThreeDScene>,
    ) -> wgpu::SurfaceTexture {
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
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
            });
        }
        let command_buffer = encoder.finish();
        self.queue.submit([command_buffer]);
        texture
    }
}
impl ThreeDimensionalRendererConstructor for Candy3DefaultRenderer {
    fn new(window: Arc<Window>) -> Self {
        Self {
            state: WgpuState::new(window),
        }
    }
}

impl Deref for Candy3DefaultRenderer {
    type Target = WgpuState;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Candy3DefaultRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
