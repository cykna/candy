use std::ops::{Deref, DerefMut};

use nalgebra::{Vector2, Vector4};
use taffy::{Size, Style};
use winit::{dpi::PhysicalSize, event::MouseButton, window::Window};

use crate::{
    renderer::{
        CandyRenderer,
        candy::CandyDefaultRenderer,
        twod::{BiDimensionalPainter, BiDimensionalRenderer},
    },
    ui::tree::tree::CandyTree,
};

#[cfg(feature = "opengl")]
use glutin::config::Config;

pub trait CandyHandler {
    #[cfg(feature = "opengl")]
    fn new(window: Window, config: Config) -> Self;
    fn draw(&mut self);
    fn resize(&mut self, size: PhysicalSize<u32>);
    fn on_mouse_move(&mut self, position: Vector2<f32>);
    fn on_press(&mut self, button: MouseButton);
    fn exit(&self);
}
pub struct CandyDefaultHandler<M> {
    mouse_pos: Vector2<f32>,
    window: Window,
    renderer: CandyDefaultRenderer,
    ui: CandyTree<M>,
}

impl<M> CandyHandler for CandyDefaultHandler<M> {
    fn new(window: Window, config: Config) -> Self {
        let mut ui: CandyTree<M> = CandyTree::new(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );

        ui.create_style(
            "pedro",
            Style {
                size: Size::from_percent(0.5, 0.5),
                ..Default::default()
            },
        );
        Self {
            ui,
            mouse_pos: Vector2::new(0.0, 0.0),
            renderer: CandyDefaultRenderer::new(&window, &config),
            window,
        }
    }
    fn on_mouse_move(&mut self, position: Vector2<f32>) {
        self.mouse_pos = position;
    }
    fn on_press(&mut self, _: MouseButton) {
        self.renderer
            .twod_renderer()
            .twod_painter()
            .background(&Vector4::new(0.0, 0.0, 0.0, 0.0));
        self.window.request_redraw();
    }
    fn draw(&mut self) {
        self.ui.render(self.renderer.twod_renderer());
        self.renderer.flush();
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        #[cfg(feature = "opengl")]
        {
            self.renderer.resize(&self.window, size.width, size.height);
        }
        self.ui.resize(size.width as f32, size.height as f32);
    }
    fn exit(&self) {}
}

impl<M> Deref for CandyDefaultHandler<M> {
    type Target = CandyTree<M>;
    fn deref(&self) -> &Self::Target {
        &self.ui
    }
}

impl<M> DerefMut for CandyDefaultHandler<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ui
    }
}
