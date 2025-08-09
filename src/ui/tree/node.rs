use slotmap::new_key_type;

use crate::{elements::CandyElement, renderer::twod::BiDimensionalPainter};

new_key_type! {pub struct CandyKey;}

///A struct that contains informations about the element it owns, as well as it's parent, children, and styling
///It's used to handle the UI tree and everything from the UI that can be defined as a N-ary Tree
pub struct CandyNode<P: BiDimensionalPainter> {
    children: Vec<CandyKey>,
    parent: Option<CandyKey>,
    inner: CandyElement<P>,
}

impl<P: BiDimensionalPainter> CandyNode<P> {
    pub fn new(inner: CandyElement<P>) -> Self {
        Self {
            children: Vec::new(),
            parent: None,
            inner,
        }
    }
    ///Retrieves this Node children
    pub fn children(&self) -> &Vec<CandyKey> {
        &self.children
    }

    ///Retrieves this Node parent
    pub fn parent(&self) -> Option<CandyKey> {
        self.parent
    }

    ///Renders the element of this node
    #[inline]
    pub fn render(&self, painter: &mut P) {
        self.inner.render(painter);
    }
}
