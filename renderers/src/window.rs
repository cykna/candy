use flume::unbounded;
use lazy_static::lazy_static;
use std::marker::PhantomData;

use nalgebra::Vector2;
use winit::{event_loop::EventLoop, window::WindowAttributes};

use crate::{
    handler::{CandyDefaultHandler, CandyHandler},
    ui::component::RootComponent,
};

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
pub struct CandyWindow<Root, T = CandyDefaultHandler<Root>>
where
    Root: RootComponent,
    T: CandyHandler<Root>,
{
    root: PhantomData<Root>,
    handler: Option<T>,
    attribs: WindowAttributes,
}
impl<Root: RootComponent, T> CandyWindow<Root, T>
where
    T: CandyHandler<Root>,
{
    pub fn new(attribs: WindowAttributes) -> Self {
        Self {
            root: PhantomData,
            handler: None,
            attribs,
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
            self.handler = Some(T::new(
                window.expect("Window not created??"),
                config,
                <Root as RootComponent>::Args::default(),
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

impl<Root, T> winit::application::ApplicationHandler<ComponentEvents> for CandyWindow<Root, T>
where
    Root: RootComponent,
    T: CandyHandler<Root>,
{
    fn resumed(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        #[cfg(not(feature = "opengl"))]
        println!("gayzinho");
    }

    fn user_event(&mut self, _: &winit::event_loop::ActiveEventLoop, event: ComponentEvents) {
        match event {
            ComponentEvents::Redraw => {
                if let Some(ref mut handler) = self.handler {
                    handler.request_redraw();
                }
            }
            ComponentEvents::CheckUpdates => {
                if let Some(ref mut handler) = self.handler {
                    if handler.root_mut().check_updates() {
                        handler.request_redraw();
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
            match event {
                winit::event::WindowEvent::RedrawRequested => {
                    handler.draw();
                }
                winit::event::WindowEvent::Resized(size) => {
                    handler.resize(size);
                }
                winit::event::WindowEvent::CloseRequested => {
                    event_loop.exit();
                    handler.exit();
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    if state.is_pressed() {
                        handler.on_press(button)
                    }
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    handler.on_mouse_move(Vector2::new(position.x as f32, position.y as f32));
                }
                winit::event::WindowEvent::MouseWheel { delta, phase, .. } => {
                    handler.on_mouse_wheel(delta, phase)
                }
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    let flag = if event.state.is_pressed() {
                        handler
                            .root_mut()
                            .keydown(event.logical_key, event.location)
                    } else {
                        handler.root_mut().keyup(event.logical_key, event.location)
                    };
                    if flag {
                        handler.request_redraw();
                    }
                }
                _ => {}
            }
        }
    }
}
