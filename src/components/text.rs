use std::ops::{Deref, DerefMut};

use nalgebra::Vector2;

use crate::{
    elements::text::CandyText,
    renderer::twod::BiDimensionalPainter,
    text::font::CandyFont,
    ui::{component::Component, styling::style::Style},
};

#[derive(Debug)]
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

    fn apply_style(&mut self, style: &dyn Style) {
        self.inner.apply_style(style);
    }
}

impl Text {
    ///Creates a new Empty Text with the specified `font`
    pub fn new(font: CandyFont) -> Self {
        Self {
            inner: CandyText::new("", Vector2::zeros(), font),
        }
    }
    ///Creates a new Text with the given `content` and using the specified `font`
    pub fn new_content(content: &str, font: CandyFont) -> Self {
        Self {
            inner: CandyText::new(content, Vector2::zeros(), font),
        }
    }
}
