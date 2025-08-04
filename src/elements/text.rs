use nalgebra::{Vector2, Vector4};

use crate::text::font::CandyFont;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TextAlignment {
    Center,
    Right,
    Left,
}

///A handlr that contains informations about an specific text
pub struct CandyText {
    font: CandyFont,
    color: Vector4<f32>,
    text: String,
    position: Vector2<f32>,
    align: TextAlignment,
}

impl CandyText {
    pub fn new(
        text: &str,
        position: Vector2<f32>,
        font: CandyFont,
        color: Vector4<f32>,
        alignment: TextAlignment,
    ) -> Self {
        Self {
            text: text.to_string(),
            position,
            font,
            color,
            align: alignment,
        }
    }

    #[inline]
    pub fn content(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn font(&self) -> &CandyFont {
        &self.font
    }

    #[inline]
    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    #[inline]
    pub fn color(&self) -> &Vector4<f32> {
        &self.color
    }

    #[inline]
    pub fn alignment(&self) -> TextAlignment {
        self.align
    }
}

///Same as `CandyText` but this is focused on drawing multiples texts at once. Normally by each of them having a different style/position
pub struct MultiText {
    texts: Vec<String>,
    position: Vector2<f32>,
}

impl MultiText {
    #[inline]
    ///Creates a new MultiText with capacity to `n` Texts
    pub fn with_capacity(n: usize, position: Vector2<f32>) -> Self {
        Self {
            texts: Vec::with_capacity(n),
            position,
        }
    }
    #[inline]
    ///Creates a empty MultiText
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            texts: Vec::new(),
            position,
        }
    }

    #[inline]
    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }
}
