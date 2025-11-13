use std::sync::Arc;

use candy_renderers::BiDimensionalPainter;
use candy_shared_types::{Rect, Style};
use nalgebra::Vector2;
use winit::{
    event::{MouseButton, MouseScrollDelta, TouchPhase},
    keyboard::{Key, KeyLocation, SmolStr},
    window::Window,
};

pub trait Component {
    ///Method called when some parent tries to resize this component. The `rect` parameter is the bounds calculated
    fn resize(&mut self, rect: Rect);
    ///Method called when this component is requested to redraw with the given `renderer`
    fn render(&self, renderer: &mut dyn BiDimensionalPainter);

    ///Applies the given `style` on this component
    fn apply_style(&mut self, style: &dyn Style);

    ///Retrieves the position of this component
    fn position(&self) -> Vector2<f32>;

    ///Retrieves the position of this component
    fn position_mut(&mut self) -> &mut Vector2<f32>;

    ///Applies the given offset to the position of this component
    fn apply_offset(&mut self, offset: Vector2<f32>) {
        *self.position_mut() += offset;
    }
}

///The root component that will be used to render all the screen. Note that mouse position is tracked by it as well
pub trait RootComponent: Component {
    type Args: Default;
    fn new(window: Arc<Window>, args: Self::Args) -> Self;

    fn window(&self) -> &Window;

    #[inline]
    ///Emitted when the mouse whell is moved `delta` is the delta of the movement
    fn on_mouse_wheel(&mut self, _: MouseScrollDelta, _: TouchPhase) -> bool {
        false
    }

    #[inline]
    ///Emitted when the mouse moves. The `position` is the new position the mouse is located at. Returns whether a redraw should be made or not
    fn on_mouse_move(&mut self, _: Vector2<f32>) -> bool {
        false
    }

    #[inline]
    ///Emitted when some click arrives. The `position` is the position of the click relative to the top left corner of the window
    ///Returns whether a redraw should be made
    fn click(&mut self, _: MouseButton) -> bool {
        false
    }

    ///Emitted when some key on the keyboard is pressed
    ///Returns whether a redraw should be made
    fn keydown(&mut self, _: Key<SmolStr>, _: KeyLocation) -> bool {
        false
    }
    ///Emitted when some key on the keyboard is released
    ///Returns whether
    fn keyup(&mut self, _: Key<SmolStr>, _: KeyLocation) -> bool {
        false
    }

    fn check_updates(&mut self) -> bool {
        false
    }
}
