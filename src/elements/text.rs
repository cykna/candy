use crate::{
    elements::DrawRule, helpers::rect::Rect, text::font::CandyFont, ui::styling::style::Style,
};
use nalgebra::{Vector2, Vector4};

#[derive(Debug, Default)]
///A handler that contains on how to draw an specific text
pub struct CandyText {
    font: CandyFont,
    text: String,
    position: Vector2<f32>,
    pub(crate) rule: DrawRule,
}

impl CandyText {
    pub fn new(text: &str, position: Vector2<f32>, font: CandyFont) -> Self {
        Self {
            text: text.to_string(),
            position,
            font,
            rule: DrawRule::new(),
        }
    }

    ///Gets the content of this text
    #[inline]
    pub fn content(&self) -> &str {
        &self.text
    }

    ///Gets the content of this text
    #[inline]
    pub fn content_mut(&mut self) -> &mut String {
        &mut self.text
    }

    ///Gets the inner font of this text
    #[inline]
    pub fn font(&self) -> &CandyFont {
        &self.font
    }

    ///Gets the inner position of this text
    pub fn position_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.position
    }

    ///Gets the inner position of this text
    #[inline]
    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }
    pub fn resize(&mut self, rect: Rect) {
        self.position.x = rect.x;
        self.position.y = rect.y;
    }

    ///Gets the bounds of this Text. XY are the position while ZW are width and height
    #[inline]
    pub fn bounds(&self) -> Rect {
        let (_, rect) = self.font.measure_str(self.content(), None);
        Rect {
            x: rect.y(),
            y: rect.y(),
            width: rect.width(),
            height: rect.height(),
        }
    }

    ///Applies the given style to this square
    #[inline]
    pub fn apply_style(&mut self, style: &impl Style) {
        self.rule.apply_style(style);
    }

    #[inline]
    pub fn with_style(mut self, style: &impl Style) -> Self {
        self.rule.apply_style(style);
        self
    }
}
