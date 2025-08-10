pub mod image;
pub mod square;
pub mod text;
use image::CandyImage;
use nalgebra::{Vector2, Vector4};
use text::CandyText;

pub use square::*;

use crate::renderer::twod::BiDimensionalPainter;

///A trait used to create custom elements.
pub trait CustomCandyElement<P: BiDimensionalPainter> {
    ///Function executed when this element is requested to be drawn
    fn render(&self, renderer: &mut P);
    ///Retrieves the position of this element
    fn position(&self) -> &Vector2<f32>;
}

///An element on the UI tree which is rendered by the `P` Painter
pub enum CandyElement<P: BiDimensionalPainter> {
    Square(CandySquare),
    Image(CandyImage<P>),
    Text(CandyText),
    Clickable {
        inner: Box<CandyElement<P>>,
        event: Box<dyn Fn(Vector2<f32>)>,
    },
    Custom(Box<dyn CustomCandyElement<P>>),
}

impl<P: BiDimensionalPainter> CandyElement<P> {
    #[inline]
    pub fn clickable<F: Fn(Vector2<f32>) + 'static>(element: CandyElement<P>, f: F) -> Self {
        Self::Clickable {
            inner: Box::new(element),
            event: Box::new(f),
        }
    }

    #[inline]
    /// Creates a new text element with the given `txt` options
    pub fn new_text(txt: CandyText) -> Self {
        Self::Text(txt)
    }

    #[inline]
    ///Creates a new image element with the given `img` options
    pub fn new_image(img: CandyImage<P>) -> Self {
        Self::Image(img)
    }

    #[inline]
    ///Creates a new square with the given options. This is equivalent to a div.
    pub fn new_square(square: CandySquare) -> Self {
        Self::Square(square)
    }

    #[inline]
    ///Creates a new custom element with the given `custom` struct that implements so
    pub fn new_custom(custom: impl CustomCandyElement<P> + 'static) -> Self {
        Self::Custom(Box::new(custom))
    }

    #[inline]
    ///Requests to the `renderer` to draw this element
    pub fn render(&self, renderer: &mut P) {
        match self {
            Self::Square(info) => renderer.square(info),
            Self::Image(info) => renderer.render_image(info),
            Self::Text(info) => renderer.text(info),
            Self::Clickable { inner, .. } => inner.render(renderer),
            Self::Custom(custom) => custom.render(renderer),
        }
    }

    ///Retrieves the position on the ui of this element
    pub fn position(&self) -> &Vector2<f32> {
        match self {
            Self::Square(s) => s.position(),
            Self::Custom(c) => c.position(),
            Self::Image(i) => i.position(),
            Self::Text(t) => t.position(),
            Self::Clickable { inner, .. } => inner.position(),
        }
    }
}

pub type Rgba = Vector4<f32>;
