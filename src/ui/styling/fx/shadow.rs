use nalgebra::{Vector2, Vector4};

use crate::ui::styling::fx::{Effect, ShadowEffect};

#[derive(Debug, Clone, Default)]
pub struct Shadow {
    color: Vector4<f32>,
    offset: Vector2<f32>,
    blur: Vector2<f32>,
}

impl Shadow {
    ///Creates a new Shadow with the provided `color`
    pub fn colored(color: Vector4<f32>) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    ///Sets the color of this shadow to be the given `color`
    pub fn with_color(mut self, color: Vector4<f32>) -> Self {
        self.color = color;
        self
    }
    #[inline]
    ///Sets the blur of this shadow to be ghe given `blur`
    pub fn with_blur(mut self, blur: Vector2<f32>) -> Self {
        self.blur = blur;
        self
    }

    #[inline]
    ///Sets the blur of this shadow to be ghe given `offset`
    pub fn with_offset(mut self, offset: Vector2<f32>) -> Self {
        self.offset = offset;
        self
    }
}

impl Effect for Shadow {
    fn shadow(&self) -> Option<super::ShadowEffect> {
        Some(ShadowEffect {
            color: self.color,
            blur: self.blur,
            offset: self.offset,
        })
    }
}
