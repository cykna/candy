use nalgebra::{Vector2, Vector4};
use slotmap::new_key_type;

use smol_str::SmolStr;

use crate::{
    elements::{CandyElement, CandySquare, text::CandyText},
    renderer::twod::BiDimensionalPainter,
    text::font::CandyFont,
    ui::component::ComponentRenderer,
};

new_key_type! {pub struct CandyKey;}

#[derive(Debug)]
///A struct that contains informations about the element it owns, as well as it's parent, children, and styling
///It's used to handle the UI tree and everything from the UI that can be defined as a N-ary Tree
pub struct CandyNode<P: BiDimensionalPainter> {
    pub(crate) inner: CandyElement<P>,
    pub(crate) children: Vec<CandyNode<P>>,
}

impl<P: BiDimensionalPainter> CandyNode<P> {
    pub fn new(inner: CandyElement<P>) -> Self {
        Self {
            inner,
            children: Vec::new(),
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<CandyNode<P>> {
        &mut self.children
    }

    pub fn render(&self, renderer: &mut P) {
        self.inner.render(renderer);
    }
    pub fn bounds(&self) -> Vector4<f32> {
        let pos = self.inner.position();
        let size = self.inner.size();
        Vector4::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y)
    }
}

///A builder for when adding a new element on the tree
pub struct ElementBuilder {
    pub(crate) inner: CandyElement<ComponentRenderer>,
    pub(crate) children: Vec<CandyNode<ComponentRenderer>>,
    pub(crate) style_name: Option<SmolStr>,
}

impl ElementBuilder {
    pub fn text(text: &str, font: CandyFont) -> Self {
        Self {
            inner: CandyElement::Text(CandyText::new(
                text,
                Vector2::zeros(),
                font,
                Vector4::new(1.0, 1.0, 1.0, 1.0),
            )),
            children: Vec::new(),
            style_name: None,
        }
    }
    #[inline]
    ///Creates a new builder for a square
    pub fn square(square: CandySquare) -> Self {
        Self {
            inner: CandyElement::Square(square),
            children: Vec::new(),
            style_name: None,
        }
    }

    #[inline]
    pub fn new(element: CandyElement<ComponentRenderer>) -> Self {
        Self {
            inner: element,
            children: Vec::new(),
            style_name: None,
        }
    }

    #[inline]
    pub fn child(mut self, child: CandyNode<ComponentRenderer>) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(mut self) -> CandyNode<ComponentRenderer> {
        let mut node = CandyNode::new(self.inner);
        node.children_mut().append(&mut self.children);
        node
    }
}
