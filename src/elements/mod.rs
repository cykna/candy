pub mod image;
pub mod square;
use image::CandyImage;
use nalgebra::Vector2;

pub use square::*;

use crate::renderer::twod::BiDimensionalPainter;

pub trait CustomCandyElement {
    fn render(&self, renderer: &mut dyn BiDimensionalPainter);
    fn position(&self) -> &Vector2<f32>;
}

pub enum CandyElement {
    Square(CandySquare),
    Image(CandyImage),
    Custom(Box<dyn CustomCandyElement>),
}

impl CandyElement {
    pub fn new_image(img: CandyImage) -> Self {
        Self::Image(img)
    }

    pub fn new_square(square: CandySquare) -> Self {
        Self::Square(square)
    }

    pub fn new_custom(custom: impl CustomCandyElement + 'static) -> Self {
        Self::Custom(Box::new(custom))
    }

    pub fn render(&mut self, renderer: &mut dyn BiDimensionalPainter) {
        match self {
            Self::Square(info) => renderer.square(info),
            Self::Image(info) => renderer.image(info),
            Self::Custom(custom) => custom.render(renderer),
        }
    }

    pub fn position(&self) -> &Vector2<f32> {
        match self {
            Self::Square(s) => s.position(),
            Self::Custom(c) => c.position(),
            Self::Image(i) => i.position(),
        }
    }
}
