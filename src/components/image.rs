use std::ops::{Deref, DerefMut};

use candy_renderers::{BiDimensionalPainter, primitives::CandyImage};
use candy_shared_types::{Rect, Style};

use crate::ui::component::Component;

#[derive(Debug)]
///A component that simply represents an image
pub struct Image {
    image: CandyImage,
}

impl Component for Image {
    fn resize(&mut self, rect: Rect) {
        self.image.resize(rect);
    }
    fn render(&self, renderer: &mut dyn BiDimensionalPainter) {
        renderer.render_image(&self.image);
    }
    fn apply_style(&mut self, style: &dyn Style) {
        self.image.apply_style(style);
    }
    fn position(&self) -> nalgebra::Vector2<f32> {
        *self.image.position()
    }
    fn position_mut(&mut self) -> &mut nalgebra::Vector2<f32> {
        self.image.position_mut()
    }
}

impl Image {
    ///Creates a new Image component from the provided `image` from candy
    pub fn new(image: CandyImage) -> Self {
        Self { image }
    }
}

impl Deref for Image {
    type Target = CandyImage;
    fn deref(&self) -> &Self::Target {
        &self.image
    }
}
impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.image
    }
}
