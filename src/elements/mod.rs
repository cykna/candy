pub mod image;
pub mod square;
pub mod text;
use image::CandyImage;
use nalgebra::Vector2;
use text::{CandyText, MultiText};

pub use square::*;

use crate::renderer::twod::BiDimensionalPainter;

pub trait CustomCandyElement {
    fn render(&self, renderer: &mut dyn BiDimensionalPainter);
    fn position(&self) -> &Vector2<f32>;
}

pub enum CandyElement {
    Square(CandySquare),
    Image(CandyImage),
    Text(CandyText),
    MultiText(MultiText),
    Custom(Box<dyn CustomCandyElement>),
}

impl CandyElement {
    #[inline]
    pub fn new_multitext(txt: MultiText) -> Self {
        Self::MultiText(txt)
    }

    #[inline]
    /// Creates a new text element with the given `txt` options
    pub fn new_text(txt: CandyText) -> Self {
        Self::Text(txt)
    }

    #[inline]
    ///Creates a new image element with the given `img` options
    pub fn new_image(img: CandyImage) -> Self {
        Self::Image(img)
    }

    #[inline]
    ///Creates a new square with the given options. This is equivalent to a div.
    pub fn new_square(square: CandySquare) -> Self {
        Self::Square(square)
    }

    #[inline]
    ///Creates a new custom element with the given `custom` struct that implements so
    pub fn new_custom(custom: impl CustomCandyElement + 'static) -> Self {
        Self::Custom(Box::new(custom))
    }

    #[inline]
    ///Requests to the `renderer` to draw this element
    pub fn render(&mut self, renderer: &mut dyn BiDimensionalPainter) {
        match self {
            Self::Square(info) => renderer.square(info),
            Self::Image(info) => renderer.image(info),
            Self::Text(info) => renderer.text(info),
            Self::MultiText(info) => renderer.multitext(info),
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
            Self::MultiText(t) => t.position(),
        }
    }
}
