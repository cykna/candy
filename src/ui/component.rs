use nalgebra::Vector2;
use winit::event::MouseButton;

use crate::{helpers::rect::Rect, renderer::twod::Candy2DRenderer};

#[cfg(any(
    feature = "default",
    feature = "opengl",
    feature = "vulkan",
    feature = "metal",
    feature = "directx"
))]
pub type ComponentRenderer = Candy2DRenderer;
#[cfg(feature = "external_renderer")]
pub type ComponentRenderer = external_renderer::UiRenderer;

pub trait Component {
    type Message;
    ///Method called when some parent tries to resize this component. The `rect` parameter is the bounds calculated
    fn resize(&mut self, rect: Rect);
    fn render(&self, renderer: &mut ComponentRenderer);
    fn on_message(&mut self, msg: Self::Message) -> Self::Message;
}

pub trait RootComponent: Default {
    type Message;
    ///Emitted when some click arrives. The `position` is the position of the click relative to the top left corner of the window
    ///Returns weather a redraw should be made
    fn click(&mut self, position: Vector2<f32>, button: MouseButton) -> bool {
        false
    }
    fn resize(&mut self, width: f32, height: f32);
    fn render(&self, renderer: &mut ComponentRenderer) -> Self::Message;
    fn on_message(&mut self, msg: Self::Message) -> Self::Message;
}
