mod bidimensional;
pub mod primitives;
mod threedimensional;

use std::sync::Arc;

pub use bidimensional::*;
#[cfg(feature = "opengl")]
use glutin::config::Config;
pub use threedimensional::*;
use vello::wgpu::CommandEncoder;
#[cfg(feature = "vello")]
use vello::wgpu::TextureView;
use winit::window::Window;

///Trait used to define renderers for Candy. It uses 2 renderers inside to draw 2D and 3D and this is used mainly for requesting commands from them
pub trait CandyRenderer {
    type TwoD: BiDimensionalRenderer + BiDimensionalRendererConstructor;
    type ThreeD: ThreeDimensionalRenderer;

    #[cfg(feature = "opengl")]
    fn new(window: Arc<Window>, config: &Config) -> Self;

    fn new(window: Arc<Window>) -> Self;

    #[cfg(feature = "opengl")]
    ///Method called when this renderer is resized. The `width` and `height` are the new dimensions that were given
    fn resize(&mut self, window: &Window, width: u32, height: u32);

    fn resize(&mut self, width: u32, height: u32);

    fn prepare(&mut self) {}

    #[cfg(not(feature = "vello"))]
    ///Used to finish every commands on the renderers. Used before rendering a new frame
    fn flush(&mut self);

    #[cfg(feature = "vello")]
    ///Used to finish every commands on the renderers. Used before rendering a new frame
    fn flush(&mut self, texture: &TextureView, encoder: CommandEncoder);

    ///Retrieves the internal renderer that controls the 2D
    fn twod_renderer(&mut self) -> &mut Self::TwoD;
    ///Retrieves the internal renderer that controls the 3D
    fn threed_renderer(&mut self) -> &mut Self::ThreeD;
}

#[derive(Debug)]
///The default renderer of a candy, used to render both 2D and 3D
pub struct CandyDefaultRenderer<TwoD = Candy2DefaultRenderer, ThreeD = Candy3DefaultRenderer> {
    twod: TwoD,
    threed: ThreeD,
}

impl<TwoD, ThreeD> CandyRenderer for CandyDefaultRenderer<TwoD, ThreeD>
where
    TwoD: BiDimensionalRenderer + BiDimensionalRendererConstructor,
    ThreeD: ThreeDimensionalRenderer + ThreeDimensionalRendererConstructor,
{
    type TwoD = TwoD;
    type ThreeD = ThreeD;
    #[cfg(feature = "opengl")]
    fn new(window: Arc<Window>, config: &Config) -> Self {
        let threed = ThreeD::new(window.clone());
        let twod = TwoD::new(window, config);
        Self { twod, threed }
    }
    #[cfg(feature = "vulkan")]
    fn new(window: Arc<Window>) -> Self {
        let threed = ThreeD::new(window.clone());
        let twod = TwoD::new(window);
        Self { twod, threed }
    }

    #[cfg(feature = "vello")]
    fn new(window: Arc<Window>) -> Self {
        let threed = ThreeD::new(window.clone());
        let state = threed.state();
        let twod = TwoD::new(window, state);
        Self { twod, threed }
    }

    #[cfg(feature = "opengl")]
    fn resize(&mut self, window: &Window, width: u32, height: u32) {
        self.twod.resize(window, width, height);
    }
    fn resize(&mut self, width: u32, height: u32) {
        #[cfg(feature = "vulkan")]
        self.twod.resize();
        self.threed.resize(width, height);
    }

    fn flush(&mut self, image: &TextureView, mut encoder: CommandEncoder) {
        self.twod.flush(image, &mut encoder);
        let buffer = encoder.finish();
        self.threed.state().queue.submit([buffer]);
    }

    fn twod_renderer(&mut self) -> &mut TwoD {
        &mut self.twod
    }
    fn threed_renderer(&mut self) -> &mut Self::ThreeD {
        &mut self.threed
    }
}
