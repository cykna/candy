use std::{fmt::Debug, sync::Arc};

use skia_safe::{
    Canvas, ColorType,
    gpu::{DirectContext, backend_render_targets, direct_contexts, surfaces, vk},
};
use vulkano::{
    Handle, Validated, VulkanError, VulkanLibrary, VulkanObject,
    device::{
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
        physical::PhysicalDeviceType,
    },
    image::{ImageUsage, view::ImageView},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions},
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{
        PresentMode, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo,
        acquire_next_image,
    },
    sync::{self, GpuFuture},
};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    window::Window,
};

use crate::{BiDimensionalRenderer, BiDimensionalRendererConstructor};

#[derive(Debug)]
struct VulkanRendererContext {
    queue: Arc<Queue>,
}

struct VulkanRenderer {
    surface: Option<skia_safe::Surface>,
    skia_ctx: DirectContext,
    last_render: Option<Box<dyn GpuFuture>>,
    swapchain_is_valid: bool,
    queue: Arc<Queue>,
    window: Arc<Window>,
    swapchain: Arc<Swapchain>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    image_index: u32,
    last_acquire: Option<SwapchainAcquireFuture>,
}
impl Drop for VulkanRenderer {
    fn drop(&mut self) {
        self.skia_ctx.abandon();
    }
}

pub struct Renderer2DEnvironment {
    context: VulkanRendererContext,
    renderer: VulkanRenderer,
}

impl VulkanRenderer {
    fn surface_for_framebuffer(
        skia_ctx: &mut skia_safe::gpu::DirectContext,
        framebuffer: Arc<Framebuffer>,
    ) -> skia_safe::Surface {
        let [width, height] = framebuffer.extent();
        let image_access = &framebuffer.attachments()[0];
        let image_object = image_access.image().handle().as_raw();

        let format = image_access.format();

        let (vk_format, color_type) = match format {
            vulkano::format::Format::B8G8R8A8_UNORM => (
                skia_safe::gpu::vk::Format::B8G8R8A8_UNORM,
                ColorType::BGRA8888,
            ),
            _ => panic!("Unsupported color format {format:?}"),
        };

        let alloc = vk::Alloc::default();
        let image_info = &unsafe {
            vk::ImageInfo::new(
                image_object as _,
                alloc,
                vk::ImageTiling::OPTIMAL,
                vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vk_format,
                1,
                None,
                None,
                None,
                None,
            )
        };

        let render_target = &backend_render_targets::make_vk(
            (width.try_into().unwrap(), height.try_into().unwrap()),
            image_info,
        );

        surfaces::wrap_backend_render_target(
            skia_ctx,
            render_target,
            skia_safe::gpu::SurfaceOrigin::TopLeft,
            color_type,
            None,
            None,
        )
        .unwrap()
    }
    pub fn invalidate_swapchain(&mut self) {
        // Typically called when the window size changes and we need to recreate framebufffers
        self.swapchain_is_valid = false;
    }

    pub fn prepare_swapchain(&mut self) {
        // It is important to call this function from time to time, otherwise resources
        // will keep accumulating and you will eventually reach an out of memory error.
        // Calling this function polls various fences in order to determine what the GPU
        // has already processed, and frees the resources that are no longer needed.
        if let Some(last_render) = self.last_render.as_mut() {
            last_render.cleanup_finished();
        }

        // Whenever the window resizes we need to recreate everything dependent on the
        // window size. In this example that includes the swapchain & the framebuffers
        let window_size: PhysicalSize<u32> = self.window.inner_size();
        if window_size.width > 0 && window_size.height > 0 && !self.swapchain_is_valid {
            // Use the new dimensions of the window.
            let (new_swapchain, new_images) = self
                .swapchain
                .recreate(SwapchainCreateInfo {
                    image_extent: window_size.into(),
                    ..self.swapchain.create_info()
                })
                .expect("failed to recreate swapchain");

            self.swapchain = new_swapchain;

            // Because framebuffers contains a reference to the old swapchain, we need to
            // recreate framebuffers as well.
            // self.framebuffers = allocate_framebuffers(&new_images, &self.render_pass);
            self.framebuffers = new_images
                .iter()
                .map(|image| {
                    let view = ImageView::new_default(image.clone()).unwrap();

                    Framebuffer::new(
                        self.render_pass.clone(),
                        FramebufferCreateInfo {
                            attachments: vec![view],
                            ..Default::default()
                        },
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();

            self.swapchain_is_valid = true;
        }
    }

    fn get_next_frame(&mut self) -> Option<(u32, SwapchainAcquireFuture)> {
        // prepare to render by identifying the next framebuffer to draw to and acquiring the
        // GpuFuture that we'll be replacing `last_render` with once we submit the frame
        let (image_index, suboptimal, acquire_future) =
            match acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    self.swapchain_is_valid = false;
                    return None;
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };

        // `acquire_next_image` can be successful, but suboptimal. This means that the
        // swapchain image will still work, but it may not display correctly. With some
        // drivers this can be when the window resizes, but it may not cause the swapchain
        // to become out of date.
        if suboptimal {
            self.swapchain_is_valid = false;
        }

        if self.swapchain_is_valid {
            Some((image_index, acquire_future))
        } else {
            None
        }
    }

    pub fn canvas(&mut self) -> &Canvas {
        if let Some(ref mut surface) = self.surface {
            surface.canvas()
        } else {
            panic!("No surface configured yet");
        }
    }

    pub fn receive_canvas(&mut self) {
        // find the next framebuffer to render into and acquire a new GpuFuture to block on
        let next_frame = self.get_next_frame().or_else(|| {
            // if suboptimal or out-of-date, recreate the swapchain and try once more
            self.prepare_swapchain();
            self.get_next_frame()
        });

        if let Some((image_index, acquire_future)) = next_frame {
            self.last_acquire = Some(acquire_future);
            self.image_index = image_index;
            // pull the appropriate framebuffer from the swapchain and attach a skia Surface to it
            let framebuffer = self.framebuffers[image_index as usize].clone();
            let surface = Self::surface_for_framebuffer(&mut self.skia_ctx, framebuffer.clone());

            self.surface = Some(surface);
            let extent: PhysicalSize<u32> = self.window.inner_size();
            let size: LogicalSize<f32> = extent.to_logical(self.window.scale_factor());
            let canvas = self.canvas();

            let scale = (
                (f64::from(extent.width) / size.width as f64) as f32,
                (f64::from(extent.height) / size.height as f64) as f32,
            );
            canvas.reset_matrix();
            canvas.scale(scale);

            // pass the suface's canvas and canvas size to the user-provided callback

            // flush the canvas's contents to the framebuffer
        }
    }
}

impl Debug for Renderer2DEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer2DEnvironment")
            .field("context", &self.context)
            .field("skia_context", &self.renderer.skia_ctx)
            .finish()
    }
}

#[derive(Debug)]
///The default renderer that is be used when drawing 2D
pub struct Candy2DefaultRenderer {
    environment: Renderer2DEnvironment,
}

impl BiDimensionalRendererConstructor for Candy2DefaultRenderer {
    fn new(window: std::sync::Arc<winit::window::Window>) -> Self {
        let libr = VulkanLibrary::new().unwrap();
        let instance = Instance::new(
            libr,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: InstanceExtensions {
                    khr_wayland_surface: true,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .unwrap();

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };

        let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

        let (device, queue) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|dev| dev.supported_extensions().contains(&device_extensions))
            .filter_map(|dev| {
                dev.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && dev.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|i| (dev, i as u32))
            })
            .min_by_key(|(p, _)| {
                // We assign a lower score to device types that are likely to be faster/better.
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                    _ => 5,
                }
            })
            .expect("No suitable physical device found");

        let (_, mut queues) = Device::new(
            device,
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: queue,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .unwrap();
        let queue = queues.next().unwrap();
        let context = VulkanRendererContext {
            queue: queue.clone(),
        };

        let renderer = {
            let size = window.inner_size();
            let device = queue.device();
            let instance = device.instance();
            let library = instance.library();
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();

            // Choosing the internal format that the images will have.
            let (image_format, _) = device
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()[0];

            // Please take a look at the docs for the meaning of the parameters we didn't mention.
            let (swapchain, _images) = Swapchain::new(
                device.clone(),
                surface,
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count,
                    image_extent: size.into(),

                    image_usage: ImageUsage::COLOR_ATTACHMENT
                        | ImageUsage::TRANSFER_SRC
                        | ImageUsage::TRANSFER_DST
                        | ImageUsage::SAMPLED
                        | ImageUsage::STORAGE,
                    image_format,

                    present_mode: PresentMode::Fifo,

                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),

                    ..Default::default()
                },
            )
            .unwrap();
            let render_pass = vulkano::single_pass_renderpass!(
                device.clone(),
                attachments: {

                    color: {

                        format: swapchain.image_format(),

                        samples: 1,

                        load_op: DontCare,

                        store_op: Store,
                    },
                },
                pass: {

                    color: [color],

                    depth_stencil: {},
                },
            )
            .unwrap();

            let framebuffers = vec![];

            let swapchain_is_valid = false;

            let last_render = Some(sync::now(device.clone()).boxed());

            let skia_ctx = unsafe {
                let get_proc = |gpo| {
                    let get_device_proc_addr = instance.fns().v1_0.get_device_proc_addr;

                    match gpo {
                        vk::GetProcOf::Instance(instance, name) => {
                            let vk_instance = ash::vk::Instance::from_raw(instance as _);
                            library.get_instance_proc_addr(vk_instance, name)
                        }
                        vk::GetProcOf::Device(device, name) => {
                            let vk_device = ash::vk::Device::from_raw(device as _);
                            get_device_proc_addr(vk_device, name)
                        }
                    }
                    .map(|f| f as _)
                    .unwrap_or_else(|| {
                        println!("Vulkan: failed to resolve {}", gpo.name().to_str().unwrap());
                        std::ptr::null()
                    })
                };

                // We then pass skia_safe references to the whole shebang, resulting in a DirectContext
                // from which we'll be able to get a canvas reference that draws directly to framebuffers
                // on the swapchain.
                let direct_context = direct_contexts::make_vulkan(
                    &vk::BackendContext::new(
                        instance.handle().as_raw() as _,
                        device.physical_device().handle().as_raw() as _,
                        device.handle().as_raw() as _,
                        (
                            queue.handle().as_raw() as _,
                            queue.queue_family_index() as usize,
                        ),
                        &get_proc,
                    ),
                    None,
                )
                .unwrap();

                direct_context
            };

            VulkanRenderer {
                surface: None,
                skia_ctx,
                queue,
                window,
                swapchain,
                swapchain_is_valid,
                render_pass,
                framebuffers,
                last_render,
                last_acquire: None,
                image_index: 0,
            }
        };
        Self {
            environment: Renderer2DEnvironment { context, renderer },
        }
    }
}

impl BiDimensionalRenderer for Candy2DefaultRenderer {
    fn flush(&mut self) {
        let renderer = &mut self.environment.renderer;
        renderer.skia_ctx.flush_and_submit();

        // send the framebuffer to the gpu and display it on screen
        renderer.last_render = renderer
            .last_render
            .take()
            .unwrap()
            .join(renderer.last_acquire.take().unwrap())
            .then_swapchain_present(
                renderer.queue.clone(),
                vulkano::swapchain::SwapchainPresentInfo::swapchain_image_index(
                    renderer.swapchain.clone(),
                    renderer.image_index,
                ),
            )
            .then_signal_fence_and_flush()
            .map(|f| Box::new(f) as _)
            .ok();
    }
    fn painter(&mut self) -> &mut dyn super::BiDimensionalPainter {
        self
    }
    fn resize(&mut self) {
        self.request_canvas();
    }
}

impl Candy2DefaultRenderer {
    pub fn canvas(&mut self) -> &Canvas {
        self.environment.renderer.canvas()
    }

    pub fn request_canvas(&mut self) {
        self.environment.renderer.receive_canvas();
    }
}
