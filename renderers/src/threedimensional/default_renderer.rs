use crate::{ThreeDimensionalRenderer, ThreeDimensionalRendererConstructor};

pub struct Candy3DefaultRenderer {}

impl ThreeDimensionalRenderer for Candy3DefaultRenderer {}
impl ThreeDimensionalRendererConstructor for Candy3DefaultRenderer {
    fn new(_: &winit::window::Window) -> Self {
        Self {}
    }
}
