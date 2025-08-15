use nalgebra::Vector4;

use crate::renderer::twod::Candy2DRenderer;

use super::{
    layout::CandyLayout,
    tree::{node::CandyNode, tree::CandyTree},
};

#[cfg(any(
    feature = "default",
    feature = "opengl",
    feature = "vulkan",
    feature = "metal",
    feature = "directx"
))]
pub type ComponentRenderer = Candy2DRenderer;
#[cfg(feature = "external_renderer")]
pub type ComponentRenderer = external_renderer::UiRenderer;

pub trait Component {
    fn new(tree: &mut CandyTree) -> Self
    where
        Self: Sized;

    fn inner(&self) -> &CandyNode<ComponentRenderer>;
    fn inner_mut(&mut self) -> &mut CandyNode<ComponentRenderer>;

    fn resize(&mut self, layout: &CandyLayout) {
        {
            let layout = layout.layout_of(self.inner().layout()).unwrap();
            self.inner_mut().resize(layout);
        }
        for child in self.children_mut() {
            child.resize(layout);
        }
    }

    fn render(&self, painter: &mut ComponentRenderer) {
        self.inner().render(painter);
        for children in self.children() {
            children.render(painter);
        }
    }
    fn bounds(&self) -> Vector4<f32> {
        self.inner().bounds()
    }
    fn children(&self) -> &Vec<Box<dyn Component>>;
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Component>>;
}
