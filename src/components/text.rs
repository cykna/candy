use std::{marker::PhantomData, ops::{Deref, DerefMut}};

use candy_renderers::{
    BiDimensionalPainter,
    primitives::{CandyFont, CandyText},
};
use candy_shared_types::{Rect, Style};
use nalgebra::Vector2;

use crate::ui::component::Component;
#[derive(Debug)]
pub struct Text<C> {
    inner: CandyText,
    phantom: PhantomData<C>
}

impl<C> Deref for Text<C> {
    type Target = CandyText;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<C> DerefMut for Text<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<Cmd:'static> Component<Cmd> for Text<Cmd> {
    #[inline]
    fn render(&self, renderer: &mut dyn BiDimensionalPainter) {
        renderer.text(&self.inner);
    }
    #[inline]
    fn resize(&mut self, rect: Rect) {
        let pos = (self as &mut dyn Component<Cmd>).position_mut();
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

impl<C> Text<C> {
    ///Creates a new Empty Text with the specified `font`
    pub fn new(font: CandyFont) -> Self {
        Self {
            inner: CandyText::new("", Vector2::zeros(), font),
            phantom:PhantomData
        }
    }

    ///Updates the inner text to be the provided `text` and recomputes it to have the correct values.
    ///Use this instead of text.content_mut(), which can be considered unsafe since it does not recompute anything and text
    ///can appear wrongly or maybe not even render on the screen due to internal clips
    #[inline]
    pub fn update_text(&mut self, text: &str) {
        self.inner.content_mut().clear();
        self.inner.content_mut().push_str(text);
        self.recompute();
    }

    ///Creates a new Text with the given `content` and using the specified `font`
    pub fn new_content(content: &str, font: CandyFont) -> Self {
        Self {
            inner: CandyText::new(content, Vector2::zeros(), font),
            phantom:PhantomData,
        }
    }
    ///Retrieves the content text used by candy for this Text
    pub fn inner(&self) -> &CandyText {
        &self.inner
    }
}
