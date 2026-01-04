use candy_renderers::{BiDimensionalRenderer, CandyRenderer};
use candy_shared_types::Rect;
use flume::unbounded;

use nalgebra::Vector2;
use winit::{event_loop::EventLoop, window::WindowAttributes};

use crate::ui::component::RootComponent;

use flume::{Receiver, Sender};

///An scheduler that sends component events. The provided `C` is the custom command the root is expecting to receive
pub(crate) struct ComponentEventsScheduler<C> {
    pub(crate) rx: Receiver<ComponentEvents<C>>,
    pub(crate) tx: Sender<ComponentEvents<C>>,
}
impl<C> ComponentEventsScheduler<C> {
    pub fn new() -> Self {        
        let (tx, rx) = unbounded::<ComponentEvents<C>>();
        ComponentEventsScheduler { rx, tx }
    }
    ///Retrieves a new sender for this scheduler
    pub fn retrieve_sender(&self) -> Sender<ComponentEvents<C>> {
        self.tx.clone()
    }
}

///Events that can be sent from some component directly to the window, such as a request to redraw due to some animation state being changed.
///This is more internal of how the lib works and in general is not known
#[derive(Debug)]
pub enum ComponentEvents<C> {
    CheckUpdates,
    Redraw,
    Custom(C)
}

impl<C> ComponentEvents<C> {
    ///Creates a new component event with the provided `command`
    pub fn new(command:C) -> Self{
        Self::Custom(command)
    }
}

unsafe impl<C> Send for ComponentEventsScheduler<C> {}
unsafe impl<C> Sync for ComponentEventsScheduler<C> {}
unsafe impl<C> Sync for ComponentEvents<C> {}
unsafe impl<C> Send for ComponentEvents<C>{}


pub struct CandyWindow<Root, Renderer, Commands>
where
    Root: RootComponent<Commands>,
    Renderer: CandyRenderer,
    Commands: 'static
{
    handler: Option<(Root, Renderer)>,
    attribs: WindowAttributes,
    scheduler: ComponentEventsScheduler<Commands>,
}
impl<Root: RootComponent<C>, R, C> CandyWindow<Root, R, C>
where
    R: CandyRenderer,
    C:'static
{
    pub fn new(attribs: WindowAttributes) -> Self {
        Self {
            handler: None,
            attribs,
            scheduler: ComponentEventsScheduler::new()
        }
    }

    pub fn run(&mut self) {
        let lp = EventLoop::with_user_event().build().unwrap();

        #[cfg(feature = "opengl")]
        {
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
            let window = window.expect("Window could not be created.");
            let renderer = CandyRenderer::new(&window, &config);
            self.handler = Some((
                Root::new(window, <Root as RootComponent<C>>::Args::default(), self.scheduler.retrieve_sender()),
                renderer,
            ));
        };
        let proxy = lp.create_proxy();
        let rx = self.scheduler.rx.clone();
        std::thread::spawn(move || {
            while let Ok(c) = rx.recv() {
                let Ok(_) = proxy.send_event(c) else {
                    println!("Thread findou. Nenhum evento de um componente será lidado mais");
                    return;
                };
            }
        });
        lp.run_app(self).unwrap();
    }
}

impl<Root, R, C> winit::application::ApplicationHandler<ComponentEvents<C>> for CandyWindow<Root, R,C>
where
    Root: RootComponent<C>,
    R: CandyRenderer,
{
    fn resumed(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        #[cfg(not(feature = "opengl"))]
        println!("gayzinho");
    }

    fn user_event(&mut self, _: &winit::event_loop::ActiveEventLoop, event: ComponentEvents<C>) {
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
            ComponentEvents::Custom(c) => if let Some(ref mut handler) = self.handler {
                handler.0.handle_command(c, &self.scheduler.tx);
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
                    handler.render(renderer.twod_renderer().painter());
                    renderer.flush();
                }
                winit::event::WindowEvent::Resized(size) => {
                    handler.resize(Rect::new(0.0, 0.0, size.width as f32, size.height as f32));
                    renderer.resize(handler.window(), size.width, size.height);
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
