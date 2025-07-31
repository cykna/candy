#[cfg(feature = "opengl")]
use glutin::config::Config;

use threed::ThreeDimensionalRenderer;
use twod::BiDimensionalRenderer;
use winit::window::Window;

pub mod candy;
pub mod threed;
pub mod twod;

pub trait CandyRenderer<TwoD: BiDimensionalRenderer, ThreeD: ThreeDimensionalRenderer> {
    #[cfg(feature = "opengl")]
    fn new(window: &Window, config: &Config) -> Self;
    #[cfg(not(feature = "opengl"))]
    fn new(window: &Window) -> Self;

    #[cfg(feature = "opengl")]
    fn resize(&mut self, window: &Window, width: u32, height: u32);
    #[cfg(not(feature = "opengl"))]
    fn resize(&mut self, width: u32, height: u32);

    fn flush(&mut self);

    fn twod_renderer(&mut self) -> &mut impl BiDimensionalRenderer;
}
