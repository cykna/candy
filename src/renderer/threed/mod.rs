use winit::window::Window;

pub mod candy3d;
pub use candy3d::Candy3DRenderer;
pub mod wgpu;

///A high level renderer with focus on 3D. It is meant to support high level things such as meshes and materials.
///The way theyre implemented, is totally based on the one which will do so.
pub trait ThreeDimensionalRenderer {
    fn new(window: &Window) -> Self;
}
