use std::fmt::Debug;

use nalgebra::Vector2;
use winit::{
    event::MouseButton,
    keyboard::{Key, KeyLocation, SmolStr},
};

use crate::{helpers::rect::Rect, renderer::twod::Candy2DRenderer, ui::styling::style::Style};

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
    ///Method called when some parent tries to resize this component. The `rect` parameter is the bounds calculated
    fn resize(&mut self, rect: Rect);
    ///Method called when this component is requested to redraw with the given `renderer`
    fn render(&self, renderer: &mut ComponentRenderer);

    ///Applies the given `style` on this component
    fn apply_style(&mut self, style: &dyn Style);

    ///Retrieves the position of this component
    fn position(&self) -> Vector2<f32>;

    ///Retrieves the position of this component
    fn position_mut(&mut self) -> &mut Vector2<f32>;
}

pub trait RootComponent: Component {
    fn new() -> Self;

    #[inline]
    ///Emitted when the mouse moves. The `position` is the new position the mouse is located at. Returns weather a redraw should be made or not
    fn on_mouse_move(&mut self, _: Vector2<f32>) -> bool {
        false
    }

    #[inline]
    ///Emitted when some click arrives. The `position` is the position of the click relative to the top left corner of the window
    ///Returns weather a redraw should be made
    fn click(&mut self, _: Vector2<f32>, _: MouseButton) -> bool {
        false
    }

    ///Emitted when some key on the keyboard is pressed
    ///Returns wather a redraw should be made
    fn keydown(&mut self, _: Key<SmolStr>, _: KeyLocation) -> bool {
        false
    }
    fn keyup(&mut self, _: Key<SmolStr>, _: KeyLocation) -> bool {
        false
    }
}
