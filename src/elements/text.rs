use crate::text::font::CandyFont;
use nalgebra::{Vector2, Vector4};
use taffy::Layout;

#[derive(Debug)]
///A handler that contains on how to draw an specific text
pub struct CandyText {
    font: CandyFont,
    color: Vector4<f32>,
    text: String,
    position: Vector2<f32>,
}

impl CandyText {
    pub fn new(text: &str, position: Vector2<f32>, font: CandyFont, color: Vector4<f32>) -> Self {
        Self {
            text: text.to_string(),
            position,
            font,
            color,
        }
    }

    ///Gets the content of this text
    #[inline]
    pub fn content(&self) -> &str {
        &self.text
    }

    ///Gets the inner font of this text
    #[inline]
    pub fn font(&self) -> &CandyFont {
        &self.font
    }

    ///Gets the inner position of this text
    #[inline]
    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }
    ///Gets the color of this text
    #[inline]
    pub fn color(&self) -> &Vector4<f32> {
        &self.color
    }

    ///Gets the bounds of this Text. XY are the position while ZW are width and height
    #[inline]
    pub fn bounds(&self) -> Vector4<f32> {
        let (_, rect) = self.font.measure_str(self.content(), None);
        Vector4::new(rect.x(), rect.y(), rect.width(), rect.height())
    }

    #[inline]
    pub fn resize(&mut self, layout: &Layout) {
        self.position.x = layout.location.x;
        self.position.y = layout.location.y;
    }
}
