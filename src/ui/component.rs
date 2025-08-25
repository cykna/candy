use nalgebra::Vector2;
use winit::event::MouseButton;

use crate::{
    elements::DrawRule,
    helpers::rect::Rect,
    renderer::twod::Candy2DRenderer,
    ui::styling::style::{DefaultStyle, Style},
};

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
    ///Method called when this component is requested to redraw with the given `renderer`
    fn render(&self, renderer: &mut ComponentRenderer);

    fn style(&self) -> impl Style {
        DefaultStyle
    }

    fn on_message(&mut self, msg: Self::Message) -> Self::Message;
}

pub trait RootComponent: Default + Component {
    fn new() -> Self;
    ///Emitted when some click arrives. The `position` is the position of the click relative to the top left corner of the window
    ///Returns weather a redraw should be made
    fn click(&mut self, _: Vector2<f32>, _: MouseButton) -> bool {
        false
    }
}
