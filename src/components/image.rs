use std::ops::{Deref, DerefMut};

use crate::{
    elements::image::CandyImage,
    renderer::{CandyRenderer, twod::BiDimensionalPainter},
    ui::component::Component,
};

#[derive(Debug)]
///A component that simply represents an image
pub struct Image<R: CandyRenderer> {
    image: CandyImage<R::TwoD>,
}

impl<R: CandyRenderer> Component<R> for Image<R> {
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.image.resize(rect);
    }
    fn render(&self, renderer: &mut R::TwoD) {
        renderer.render_image(&self.image);
    }
    fn apply_style(&mut self, style: &dyn crate::ui::styling::style::Style) {
        self.image.apply_style(style);
    }
    fn position(&self) -> nalgebra::Vector2<f32> {
        *self.image.position()
    }
    fn position_mut(&mut self) -> &mut nalgebra::Vector2<f32> {
        self.image.position_mut()
    }
}

impl<R: CandyRenderer> Image<R> {
    ///Creates a new Image component from the provided `image` from candy
    pub fn new(image: CandyImage<R::TwoD>) -> Self {
        Self { image }
    }
}

impl<R> Deref for Image<R>
where
    R: CandyRenderer,
{
    type Target = CandyImage<R::TwoD>;
    fn deref(&self) -> &Self::Target {
        &self.image
    }
}
impl<R> DerefMut for Image<R>
where
    R: CandyRenderer,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.image
    }
}
