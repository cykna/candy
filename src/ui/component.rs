use taffy::NodeId;

use crate::renderer::twod::Candy2DRenderer;

use super::tree::{
    node::{CandyKey, CandyNode},
    tree::CandyTree,
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

pub trait Component<M, S: Default = ()> {
    fn new(tree: &mut CandyTree<M>, parent: Option<CandyKey>) -> Self
    where
        Self: Sized;

    fn layout(&self) -> NodeId;
    fn render(&self, ui: &CandyTree<M>) -> CandyNode<ComponentRenderer>;
    fn children(&self) -> &Vec<CandyKey>;
    fn parent(&self) -> Option<CandyKey>;

    fn on_message(&mut self, msg: M) -> M;
}
