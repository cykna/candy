use winit::window::Window;
mod default_renderer;
pub use default_renderer::*;
pub trait ThreeDimensionalRenderer {}
pub trait ThreeDimensionalRendererConstructor {
    fn new(window: &Window) -> Self;
}
