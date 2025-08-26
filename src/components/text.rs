use std::ops::{Deref, DerefMut};

use nalgebra::Vector2;

use crate::{
    elements::text::CandyText,
    renderer::twod::BiDimensionalPainter,
    text::font::CandyFont,
    ui::{component::Component, styling::style::Style},
};

pub struct Text {
    inner: CandyText,
}

impl Deref for Text {
    type Target = CandyText;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Component for Text {
    #[inline]
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        renderer.text(&self.inner);
    }
    #[inline]
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        let pos = self.position_mut();
        pos.x = rect.x;
        pos.y = rect.y;
    }

    fn apply_style<S>(&mut self, style: S)
    where
        S: Style,
    {
        self.inner.apply_style(&style);
    }
}

impl Text {
    pub fn new(font: CandyFont, style: impl Style + 'static) -> Self {
        Self {
            inner: CandyText::new("", Vector2::zeros(), font).with_style(&style),
        }
    }
}
