use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use nalgebra::Vector2;

use crate::{
    elements::text::CandyText,
    renderer::{CandyRenderer, candy::CandyDefaultRenderer, twod::BiDimensionalPainter},
    text::font::CandyFont,
    ui::{component::Component, styling::style::Style},
};
#[derive(Debug)]
pub struct Text<R: CandyRenderer = CandyDefaultRenderer> {
    inner: CandyText,
    phantom: PhantomData<R>,
}

impl<R: CandyRenderer> Deref for Text<R> {
    type Target = CandyText;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<R: CandyRenderer> DerefMut for Text<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<R: CandyRenderer> Component<R> for Text<R> {
    #[inline]
    fn render(&self, renderer: &mut R::TwoD) {
        renderer.text(&self.inner);
    }
    #[inline]
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        let pos = self.position_mut();
        pos.x = rect.x;
        pos.y = rect.y;
    }

    fn apply_style(&mut self, style: &dyn Style) {
        self.inner.apply_style(style);
    }

    fn position(&self) -> Vector2<f32> {
        *self.inner.position()
    }
    fn position_mut(&mut self) -> &mut Vector2<f32> {
        self.inner.position_mut()
    }
    fn apply_offset(&mut self, offset: Vector2<f32>) {
        *self.inner.position_mut() += offset;
    }
}

impl<R: CandyRenderer> Text<R> {
    ///Creates a new Empty Text with the specified `font`
    pub fn new(font: CandyFont) -> Self {
        Self {
            inner: CandyText::new("", Vector2::zeros(), font),
            phantom: PhantomData,
        }
    }
    ///Creates a new Text with the given `content` and using the specified `font`
    pub fn new_content(content: &str, font: CandyFont) -> Self {
        Self {
            inner: CandyText::new(content, Vector2::zeros(), font),
            phantom: PhantomData,
        }
    }
    ///Retrieves the content text used by candy for this Text
    pub fn inner(&self) -> &CandyText {
        &self.inner
    }
}
