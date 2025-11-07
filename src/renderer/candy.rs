use glutin::config::Config;
use winit::window::Window;

use crate::renderer::twod::BiDimensionalRendererConstructor;

use super::{
    CandyRenderer,
    threed::{Candy3DRenderer, ThreeDimensionalRenderer},
    twod::{BiDimensionalRenderer, Candy2DRenderer},
};

#[derive(Debug)]
///The default renderer of a candy, used to render both 2D and 3D
pub struct CandyDefaultRenderer<TwoD = Candy2DRenderer, ThreeD = Candy3DRenderer> {
    twod: TwoD,
    threed: ThreeD,
}

impl<TwoD, ThreeD> CandyRenderer for CandyDefaultRenderer<TwoD, ThreeD>
where
    TwoD: BiDimensionalRenderer + BiDimensionalRendererConstructor,
    ThreeD: ThreeDimensionalRenderer,
{
    type TwoD = TwoD;
    type ThreeD = ThreeD;
    #[cfg(feature = "opengl")]
    fn new(window: &Window, config: &Config) -> Self {
        Self {
            twod: TwoD::new(window, config),
            threed: ThreeD::new(window),
        }
    }
    #[cfg(feature = "opengl")]
    fn resize(&mut self, window: &Window, width: u32, height: u32) {
        self.twod.resize(window, width, height);
    }

    fn flush(&mut self) {
        self.twod.flush();
    }

    fn twod_renderer(&mut self) -> &mut TwoD {
        &mut self.twod
    }
}
