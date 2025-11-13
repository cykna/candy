use candy_renderers::{BiDimensionalRenderer, CandyRenderer, ThreeDimensionalRenderer};
use candy_shared_types::Rect;
use flume::unbounded;
use lazy_static::lazy_static;

use nalgebra::Vector2;
use winit::{event_loop::EventLoop, window::WindowAttributes};

use crate::ui::component::RootComponent;

use flume::{Receiver, Sender};

lazy_static! {
    pub(crate) static ref SCHEDULER: ComponentEventsScheduler = {
        let (tx, rx) = unbounded::<ComponentEvents>();
        ComponentEventsScheduler { rx, tx }
    };
}

pub(crate) struct ComponentEventsScheduler {
    pub(crate) rx: Receiver<ComponentEvents>,
    pub(crate) tx: Sender<ComponentEvents>,
}
impl ComponentEventsScheduler {
    ///Retrieves a new sender for this scheduler
    pub fn retrieve_sender(&self) -> Sender<ComponentEvents> {
        self.tx.clone()
    }
}

///Events that can be sent from some component directly to the window, such as a request to redraw due to some animation state being changed.
///This is more internal of how the lib works and in general is not known
#[derive(Debug)]
pub(crate) enum ComponentEvents {
    CheckUpdates,
    Redraw,
}

unsafe impl Send for ComponentEventsScheduler {}
unsafe impl Sync for ComponentEventsScheduler {}
unsafe impl Sync for ComponentEvents {}
unsafe impl Send for ComponentEvents {}

#[derive(Default, Debug)]
pub struct CandyWindow<Root, Renderer>
where
    Root: RootComponent,
    Renderer: CandyRenderer,
{
    handler: Option<(Root, Renderer)>,
    attribs: WindowAttributes,
}
impl<Root: RootComponent, R> CandyWindow<Root, R>
where
    R: CandyRenderer,
{
    pub fn new(attribs: WindowAttributes) -> Self {
        Self {
            handler: None,
            attribs,
        }
    }

    pub fn run(&mut self) {
        let lp = EventLoop::with_user_event().build().unwrap();
        #[cfg(feature = "opengl")]
        {
            use std::sync::Arc;

            use glutin::config::{ConfigTemplateBuilder, GlConfig};

            use glutin_winit::DisplayBuilder;
            let template = ConfigTemplateBuilder::new()
                .with_alpha_size(8)
                .with_transparency(true);
            let (window, config) = DisplayBuilder::new()
                .with_window_attributes(Some(self.attribs.clone()))
                .build(&lp, template, |configs| {
                    configs
                        .reduce(|accum, config| {
                            let transparency_check =
                                config.supports_transparency().unwrap_or(false)
                                    && !accum.supports_transparency().unwrap_or(false);
                            if transparency_check || config.num_samples() < accum.num_samples() {
                                config
                            } else {
                                accum
                            }
                        })
                        .unwrap()
                })
                .unwrap();
            let window = Arc::new(window.expect("Window could not be created."));

            let renderer = CandyRenderer::new(window.clone(), &config);
            self.handler = Some((
                Root::new(window, <Root as RootComponent>::Args::default()),
                renderer,
            ));
        };

        let proxy = lp.create_proxy();

        std::thread::spawn(move || {
            while let Ok(c) = SCHEDULER.rx.recv() {
                let Ok(_) = proxy.send_event(c) else {
                    println!("Thread findou. Nenhum evento de um componente será lidado mais");
                    return;
                };
            }
        });
        lp.run_app(self).unwrap();
    }
}

impl<Root, R> winit::application::ApplicationHandler<ComponentEvents> for CandyWindow<Root, R>
where
    Root: RootComponent,
    R: CandyRenderer,
{
    fn resumed(&mut self, active: &winit::event_loop::ActiveEventLoop) {
        #[cfg(any(feature = "vulkan", feature = "vello"))]
        {
            use std::sync::Arc;

            let window = active.create_window(self.attribs.clone()).unwrap();
            let window = Arc::new(window);

            let renderer = CandyRenderer::new(window.clone());
            self.handler = Some((
                Root::new(window, <Root as RootComponent>::Args::default()),
                renderer,
            ));
        }
    }

    fn user_event(&mut self, _: &winit::event_loop::ActiveEventLoop, event: ComponentEvents) {
        match event {
            ComponentEvents::Redraw => {
                if let Some(ref mut handler) = self.handler {
                    let (handler, _) = (&mut handler.0, &mut handler.1);
                    handler.window().request_redraw();
                }
            }
            ComponentEvents::CheckUpdates => {
                if let Some(ref mut handler) = self.handler {
                    let (handler, _) = (&mut handler.0, &mut handler.1);
                    if handler.check_updates() {
                        handler.window().request_redraw();
                    };
                }
            }
        }
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(ref mut handler) = self.handler {
            let (handler, renderer) = (&mut handler.0, &mut handler.1);
            match event {
                winit::event::WindowEvent::RedrawRequested => {
                    let (texture, view, encoder) = renderer.threed_renderer().render(None);
                    handler.render(renderer.twod_renderer().painter());
                    #[cfg(feature = "vello")]
                    {
                        renderer.flush(&view, encoder);
                    }
                    #[cfg(not(feature = "vello"))]
                    {
                        renderer.flush();
                    }
                    texture.present();
                }
                winit::event::WindowEvent::Resized(size) => {
                    handler.resize(Rect::new(0.0, 0.0, size.width as f32, size.height as f32));
                    #[cfg(feature = "opengl")]
                    renderer.resize(handler.window(), size.width, size.height);
                    #[cfg(feature = "vulkan")]
                    renderer.resize(size.width, size.height);
                }
                winit::event::WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    if state.is_pressed() && handler.click(button) {
                        handler.window().request_redraw();
                    }
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    if handler.on_mouse_move(Vector2::new(position.x as f32, position.y as f32)) {
                        handler.window().request_redraw();
                    }
                }
                winit::event::WindowEvent::MouseWheel { delta, phase, .. } => {
                    if handler.on_mouse_wheel(delta, phase) {
                        handler.window().request_redraw();
                    }
                }
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    let flag = if event.state.is_pressed() {
                        handler.keydown(event.logical_key, event.location)
                    } else {
                        handler.keyup(event.logical_key, event.location)
                    };
                    if flag {
                        handler.window().request_redraw();
                    }
                }
                _ => {}
            }
        }
    }
}
