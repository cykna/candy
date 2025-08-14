use nalgebra::{Vector2, Vector4};
use slotmap::new_key_type;

use smol_str::SmolStr;
use taffy::{Layout, NodeId, Style};

use crate::{
    elements::{CandyElement, CandySquare, text::CandyText},
    renderer::twod::BiDimensionalPainter,
    text::font::CandyFont,
};

new_key_type! {pub struct CandyKey;}

#[derive(Debug)]
///A struct that contains informations about the element it owns, as well as it's parent, children, and styling
///It's used to handle the UI tree and everything from the UI that can be defined as a N-ary Tree
pub struct CandyNode<P: BiDimensionalPainter> {
    children: Vec<CandyKey>,
    parent: Option<CandyKey>,
    pub(crate) inner: CandyElement<P>,
    style: NodeId,
}

impl<P: BiDimensionalPainter> CandyNode<P> {
    pub fn new(inner: CandyElement<P>, style: NodeId) -> Self {
        Self {
            children: Vec::new(),
            parent: None,
            inner,
            style,
        }
    }

    pub fn style(&self) -> NodeId {
        self.style
    }

    ///Removes the child with the given `key` and returns weather it was sucessfully removed or not
    pub fn remove_child(&mut self, key: CandyKey) -> bool {
        if let Some(index) = self.children.iter().position(|child| *child == key) {
            self.children.swap_remove(index);
            true
        } else {
            false
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
    pub fn children_mut(&mut self) -> &mut Vec<CandyKey> {
        &mut self.children
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

    #[inline]
    pub fn resize(&mut self, layout: &Layout) {
        self.inner.resize(layout);
    }

    #[inline]
    ///Gets the bounds of this element, XY are the XY position, zw are width and height
    pub fn bounds(&self) -> Vector4<f32> {
        let pos = self.inner.position();
        let size = self.inner.size();
        Vector4::new(pos.x, pos.y, size.x, size.y)
    }
}

///A builder for when adding a new element on the tree
pub struct ElementBuilder<P: BiDimensionalPainter> {
    pub(crate) children: Vec<ElementBuilder<P>>,
    pub(crate) inner: CandyElement<P>,
    pub(crate) style_name: Option<SmolStr>,
    pub(crate) styled: Style,
}

impl<P: BiDimensionalPainter> ElementBuilder<P> {
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
            styled: Style::default(),
        }
    }
    #[inline]
    ///Creates a new builder for a square
    pub fn square(square: CandySquare) -> Self {
        Self {
            inner: CandyElement::Square(square),
            children: Vec::new(),
            style_name: None,
            styled: Style::default(),
        }
    }

    #[inline]
    pub fn new(element: CandyElement<P>) -> Self {
        Self {
            inner: element,
            children: Vec::new(),
            style_name: None,
            styled: Style::default(),
        }
    }

    ///Appends the given elements on this builder
    pub fn children(mut self, mut children: Vec<ElementBuilder<P>>) -> Self {
        self.children.append(&mut children);
        self
    }

    pub fn classed(mut self, style: &str) -> Self {
        self.style_name = Some(style.into());
        self
    }

    pub fn styled(mut self, style: Style) -> Self {
        self.styled = style;
        self
    }
}
