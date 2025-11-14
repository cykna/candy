use candy_renderers::WgpuState;
use candy_shared_types::threed::Mesh;
use nalgebra::Vector2;
use winit::{
    event::{MouseButton, MouseScrollDelta, TouchPhase},
    keyboard::{Key, KeyLocation, SmolStr},
};

///A Scene that is used to be rendered. This may contain all the informations about the meshes
pub trait ThreeDScene {
    fn new(state: &WgpuState) -> Self;
    fn insert_mesh(&mut self, mesh: Mesh);
    fn meshes(&self) -> impl Iterator<Item = &Mesh>;
    #[inline]
    ///Emitted when the mouse whell is moved `delta` is the delta of the movement
    fn on_mouse_wheel(&mut self, state: &WgpuState, _: MouseScrollDelta, _: TouchPhase) -> bool {
        false
    }

    #[inline]
    ///Emitted when the mouse moves. The `position` is the new position the mouse is located at. Returns whether a redraw should be made or not
    fn on_mouse_move(&mut self, state: &WgpuState, _: Vector2<f32>) -> bool {
        false
    }

    #[inline]
    ///Emitted when some click arrives. The `position` is the position of the click relative to the top left corner of the window
    ///Returns whether a redraw should be made
    fn click(&mut self, state: &WgpuState, _: MouseButton) -> bool {
        false
    }

    ///Emitted when some key on the keyboard is pressed
    ///Returns whether a redraw should be made
    fn keydown(&mut self, state: &WgpuState, _: Key<SmolStr>, _: KeyLocation) -> bool {
        false
    }
    ///Emitted when some key on the keyboard is released
    ///Returns whether
    fn keyup(&mut self, state: &WgpuState, _: Key<SmolStr>, _: KeyLocation) -> bool {
        false
    }
    ///Emitted when the window is resized. `width` and `height` are the new dimentions of it
    fn resize(&mut self, width: u32, height: u32) -> bool {
        false
    }
}
