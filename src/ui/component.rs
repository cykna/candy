use candy_renderers::BiDimensionalPainter;
use candy_shared_types::{Rect, Style};
use flume::Sender;
use nalgebra::Vector2;
use winit::{
    event::{MouseButton, MouseScrollDelta, TouchPhase},
    keyboard::{Key, KeyLocation, SmolStr},
    window::Window,
};

use crate::window::ComponentEvents;

pub trait Component<C:'static> {
    ///Method called when some parent tries to resize this component. The `rect` parameter is the bounds calculated
    fn resize(&mut self, rect: Rect);
    ///Method called when this component is requested to redraw with the given `renderer`
    fn render(&self, renderer: &mut dyn BiDimensionalPainter);

    ///Applies the given `style` on this component
    fn apply_style(&mut self, style: &dyn Style);

    ///Retrieves the position of this component
    fn position(&self) -> Vector2<f32>{
        Vector2::zeros()
    }

    ///Retrieves the position of this component
    fn position_mut(&mut self) -> &mut Vector2<f32>;

    ///Applies the given offset to the position of this component
    fn apply_offset(&mut self, offset: Vector2<f32>) {
        *self.position_mut() += offset;
    }
    fn handle_command(&mut self, _:C, _: &Sender<ComponentEvents<C>>){}
}

///The root component that will be used to render all the screen. Note that mouse position is tracked by it as well
pub trait RootComponent<Command>: Component<Command> where Command:'static {
    type Args: Default;
    ///Creates a new root component with the provided `window`, `args` and `sender`.
    ///The `sender` argument is used mainly for sending commands to the proxy of winit, so handle command will be called
    fn new(window: Window, args: Self::Args, sender:Sender<ComponentEvents<Command>>) -> Self;

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
