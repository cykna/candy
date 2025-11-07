use lazy_static::lazy_static;
use std::marker::PhantomData;

use nalgebra::Vector2;
use winit::{
    event_loop::{EventLoop, EventLoopProxy},
    window::WindowAttributes,
};

use crate::{
    handler::{CandyDefaultHandler, CandyHandler},
    renderer::{CandyRenderer, candy::CandyDefaultRenderer},
    ui::component::RootComponent,
};

use std::sync::mpsc::{Receiver, Sender, channel};

lazy_static! {
    static ref SCHEDULER: ComponentEventsScheduler = {
        let (tx, rx) = channel::<ComponentEvents>();
        ComponentEventsScheduler { rx, tx }
    };
}

pub(crate) struct ComponentEventsScheduler {
    pub(crate) rx: Receiver<ComponentEvents>,
    pub(crate) tx: Sender<ComponentEvents>,
}

///Events that can be sent from some component directly to the window, such as a request to redraw due to some animation state being changed.
///This is more internal of how the lib works and in general is not known
#[derive(Debug)]
pub(crate) enum ComponentEvents {
    Redraw,
}

unsafe impl Send for ComponentEventsScheduler {}
unsafe impl Sync for ComponentEventsScheduler {}
unsafe impl Sync for ComponentEvents {}
unsafe impl Send for ComponentEvents {}

#[derive(Default, Debug)]
pub struct CandyWindow<
    Root,
    Renderer = CandyDefaultRenderer,
    T = CandyDefaultHandler<Root, Renderer>,
> where
    Root: RootComponent<Renderer>,
    Renderer: CandyRenderer,
    T: CandyHandler<Root, Renderer>,
{
    root: PhantomData<(Root, Renderer)>,
    handler: Option<T>,
    proxy: Option<EventLoopProxy<ComponentEvents>>,
    attribs: WindowAttributes,
}
impl<Renderer: CandyRenderer, Root: RootComponent<Renderer>, T> CandyWindow<Root, Renderer, T>
where
    T: CandyHandler<Root, Renderer>,
{
    pub fn new(attribs: WindowAttributes) -> Self {
        Self {
            root: PhantomData,
            handler: None,
            attribs,
            proxy: None,
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
                <Root as RootComponent<Renderer>>::Args::default(),
            ));
        };
        self.proxy = Some(lp.create_proxy());
        lp.run_app(self).unwrap();
    }
}

impl<Root, Renderer, T> winit::application::ApplicationHandler<ComponentEvents>
    for CandyWindow<Root, Renderer, T>
where
    Root: RootComponent<Renderer>,
    Renderer: CandyRenderer,
    T: CandyHandler<Root, Renderer>,
{
    fn resumed(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        #[cfg(not(feature = "opengl"))]
        println!("gayzinho");
    }
    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        if let Some(ref mut proxy) = self.proxy {
            while let Ok(c) = SCHEDULER.rx.try_recv() {
                proxy.send_event(c).unwrap();
            }
        }
    }
    fn user_event(&mut self, _: &winit::event_loop::ActiveEventLoop, event: ComponentEvents) {
        match event {
            ComponentEvents::Redraw => {
                if let Some(ref mut handler) = self.handler {
                    handler.request_redraw();
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
