use std::ops::{Deref, DerefMut};

use crate::{
    elements::image::CandyImage, renderer::twod::BiDimensionalPainter, ui::component::Component,
};

pub struct Image {
    image: CandyImage,
}

impl Component for Image {
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.image.resize(rect);
    }
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        renderer.render_image(&self.image);
    }
    fn apply_style(&mut self, style: &dyn crate::ui::styling::style::Style) {
        self.image.apply_style(style);
    }
}

impl Image {
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
