use nalgebra::Vector2;

use winit::{
    dpi::PhysicalSize,
    event::{MouseButton, MouseScrollDelta, TouchPhase},
    window::Window,
};

use crate::{
    renderer::{
        CandyRenderer, candy::CandyDefaultRenderer, threed::ThreeDimensionalRenderer,
        twod::BiDimensionalRenderer,
    },
    ui::component::RootComponent,
};

#[cfg(feature = "opengl")]
use glutin::config::Config;

pub trait CandyHandler<Root>
where
    Root: RootComponent,
{
    #[cfg(feature = "opengl")]
    fn new(window: Window, config: Config, arg: <Root as RootComponent>::Args) -> Self;
    fn root(&self) -> &Root;
    fn root_mut(&mut self) -> &mut Root;
    fn draw(&mut self);
    fn resize(&mut self, size: PhysicalSize<u32>);
    fn on_mouse_move(&mut self, position: Vector2<f32>);
    fn on_mouse_wheel(&mut self, delta: MouseScrollDelta, phase: TouchPhase);
    fn on_press(&mut self, button: MouseButton);

    fn request_redraw(&self);

    fn exit(&self);
}
pub struct CandyDefaultHandler<Root, Renderer: CandyRenderer = CandyDefaultRenderer>
where
    Root: RootComponent,
{
    mouse_pos: Vector2<f32>,
    window: Window,
    renderer: Renderer,
    root: Root,
}

impl<Root, Renderer> CandyHandler<Root> for CandyDefaultHandler<Root, Renderer>
where
    Renderer: CandyRenderer,
    Root: RootComponent,
    <Renderer as CandyRenderer>::TwoD: BiDimensionalRenderer,
    <Renderer as CandyRenderer>::ThreeD: ThreeDimensionalRenderer,
{
    fn root(&self) -> &Root {
        &self.root
    }
    fn root_mut(&mut self) -> &mut Root {
        &mut self.root
    }
    fn new(window: Window, config: Config, arg: <Root as RootComponent>::Args) -> Self {
        Self {
            mouse_pos: Vector2::new(0.0, 0.0),
            renderer: Renderer::new(&window, &config),
            window,
            root: Root::new(arg),
        }
    }

    fn request_redraw(&self) {
        self.window.request_redraw();
    }

    #[inline]
    fn on_mouse_move(&mut self, position: Vector2<f32>) {
        self.mouse_pos = position;
        if self.root.on_mouse_move(position) {
            self.window.request_redraw();
        }
    }
    fn on_mouse_wheel(&mut self, delta: MouseScrollDelta, phase: TouchPhase) {
        if self.root.on_mouse_wheel(delta, phase, self.mouse_pos) {
            self.window.request_redraw();
        }
    }
    fn on_press(&mut self, btn: MouseButton) {
        if self.root.click(self.mouse_pos, btn) {
            self.window.request_redraw();
        };
    }

    fn draw(&mut self) {
        self.root.render(self.renderer.twod_renderer().painter());
        self.renderer.flush();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.root.resize(crate::helpers::rect::Rect {
            x: 0.0,
            y: 0.0,
            width: size.width as f32,
            height: size.height as f32,
        });
        #[cfg(feature = "opengl")]
        {
            self.renderer.resize(&self.window, size.width, size.height);
        }
    }
    fn exit(&self) {}
}
