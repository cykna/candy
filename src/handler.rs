use nalgebra::{Vector2, Vector4};

use winit::{dpi::PhysicalSize, event::MouseButton, window::Window};

use crate::{
    renderer::{
        CandyRenderer,
        candy::CandyDefaultRenderer,
        twod::{BiDimensionalPainter, BiDimensionalRenderer},
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
    fn new(window: Window, config: Config) -> Self;
    fn root(&self) -> &Root;
    fn root_mut(&mut self) -> &mut Root;
    fn draw(&mut self);
    fn resize(&mut self, size: PhysicalSize<u32>);
    fn on_mouse_move(&mut self, position: Vector2<f32>);
    fn on_press(&mut self, button: MouseButton);
    fn exit(&self);
}
pub struct CandyDefaultHandler<Root>
where
    Root: RootComponent,
{
    mouse_pos: Vector2<f32>,
    window: Window,
    renderer: CandyDefaultRenderer,
    root: Root,
}

impl<Root> CandyHandler<Root> for CandyDefaultHandler<Root>
where
    Root: RootComponent,
{
    fn root(&self) -> &Root {
        &self.root
    }
    fn root_mut(&mut self) -> &mut Root {
        &mut self.root
    }
    fn new(window: Window, config: Config) -> Self {
        Self {
            mouse_pos: Vector2::new(0.0, 0.0),
            renderer: CandyDefaultRenderer::new(&window, &config),
            window,
            root: Root::default(),
        }
    }
    #[inline]
    fn on_mouse_move(&mut self, position: Vector2<f32>) {
        self.mouse_pos = position;
    }
    fn on_press(&mut self, btn: MouseButton) {
        if self.root.click(self.mouse_pos, btn) {
            self.window.request_redraw();
        };
    }
    fn draw(&mut self) {
        self.root.render(self.renderer.twod_renderer());
        self.renderer.flush();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.root.resize(size.width as f32, size.height as f32);
        #[cfg(feature = "opengl")]
        {
            self.renderer.resize(&self.window, size.width, size.height);
        }
    }
    fn exit(&self) {}
}
