extern crate wgpu;
use std::sync::Arc;

use candy_shared_types::threed::{GpuCalculation, ThreeDScene};
use wgpu::SurfaceTexture;
use winit::window::Window;
mod default_renderer;
pub use default_renderer::*;
pub trait ThreeDimensionalRenderer {
    ///This is executed when window this renderer renders to resizes. The `width` and `height` are the new sizes of it
    fn resize(&mut self, width: u32, height: u32);

    ///Renders the 3D with the provided scene and returns the surface texture to be able to draw other things after it, such as skia
    fn render(&mut self, scene: Option<&dyn ThreeDScene>) -> SurfaceTexture;

    fn calculate(&mut self, _: &dyn GpuCalculation) {}
}
pub trait ThreeDimensionalRendererConstructor {
    fn new(window: Arc<Window>) -> Self;
}
