use slotmap::new_key_type;

use crate::{
    elements::{CandyElement, CandySquare},
    renderer::twod::BiDimensionalPainter,
};

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

    ///Adds the given `children` to be the children of this element
    pub fn add_children(&mut self, mut children: Vec<CandyKey>) {
        self.children.append(&mut children);
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

///A builder for when adding a new element on the tree
pub struct ElementBuilder<P: BiDimensionalPainter> {
    pub(crate) children: Vec<ElementBuilder<P>>,
    pub(crate) inner: CandyElement<P>,
}

impl<P: BiDimensionalPainter> ElementBuilder<P> {
    #[inline]
    ///Creates a new builder for a square
    pub fn square(square: CandySquare) -> Self {
        Self {
            inner: CandyElement::Square(square),
            children: Vec::new(),
        }
    }

    #[inline]
    pub fn new(element: CandyElement<P>) -> Self {
        Self {
            inner: element,
            children: Vec::new(),
        }
    }

    ///Appends the given elements on this builder
    pub fn children(mut self, mut children: Vec<ElementBuilder<P>>) -> Self {
        self.children.append(&mut children);
        self
    }
}
