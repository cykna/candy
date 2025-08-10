#[cfg(feature = "opengl")]
use glutin::config::Config;

use threed::ThreeDimensionalRenderer;
use twod::BiDimensionalRenderer;
use winit::window::Window;

pub mod candy;
pub mod threed;
pub mod twod;

///Trait used to define renderers for Candy. It uses 2 renderers inside to draw 2D and 3D and this is used mainly for requesting commands from them
pub trait CandyRenderer<TwoD: BiDimensionalRenderer, ThreeD: ThreeDimensionalRenderer> {
    #[cfg(feature = "opengl")]
    fn new(window: &Window, config: &Config) -> Self;
    #[cfg(not(feature = "opengl"))]
    fn new(window: &Window) -> Self;

    ///Method called when this renderer is resized. The `width` and `height` are the new dimensions that were given
    #[cfg(feature = "opengl")]
    fn resize(&mut self, window: &Window, width: u32, height: u32);
    #[cfg(not(feature = "opengl"))]
    fn resize(&mut self, width: u32, height: u32);

    ///Used to finish every commands on the renderers. Used before rendering a new fram
    fn flush(&mut self);

    ///Retrieves the internal renderer that controls the 2D
    fn twod_renderer(&mut self) -> &mut TwoD;
}
