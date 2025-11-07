use super::ThreeDimensionalRenderer;
use crate::renderer::threed::wgpu::WgpuCore;
use winit::window::Window;

#[derive(Debug)]
pub struct Candy3DRenderer {
    wgpu_core: WgpuCore,
}

impl ThreeDimensionalRenderer for Candy3DRenderer {
    fn new(_: &Window) -> Self {
        Self {
            wgpu_core: WgpuCore {},
        }
    }
}
