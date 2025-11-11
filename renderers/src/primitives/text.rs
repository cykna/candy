use crate::{
    elements::DrawRule,
    helpers::{rect::Rect, vec4f32_to_color},
    text::font::CandyFont,
    ui::styling::style::Style,
};
use nalgebra::{Vector2, Vector4};

#[derive(Debug, Default)]
///A handler that contains on how to draw an specific text
pub struct CandyText {
    font: CandyFont,
    text: String,
    position: Vector2<f32>,
    size: Vector2<f32>,
    pub(crate) rule: DrawRule,
}

impl CandyText {
    pub fn new(text: &str, position: Vector2<f32>, font: CandyFont) -> Self {
        Self {
            text: text.to_string(),
            size: Vector2::zeros(),
            position,
            font,
            rule: {
                let mut rule = DrawRule::new();
                rule.set_color(&Vector4::new(1.0, 1.0, 1.0, 1.0));
                rule
            },
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
    pub fn size_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.size
    }

    ///Gets the inner position of this text
    #[inline]
    pub fn size(&self) -> &Vector2<f32> {
        &self.size
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

    #[inline]
    pub fn resize(&mut self, rect: Rect) {
        self.position.x = rect.x;
        self.position.y = rect.y;
        self.size.x = rect.width;
        self.size.y = rect.height;
    }

    ///Gets the width of this Text
    pub fn text_width(&self) -> f32 {
        self.font
            .measure_str(&self.content(), Some(&self.rule.inner))
            .0
    }

    ///Gets the bounds of this Text.
    #[inline]
    pub fn text_bounds(&self) -> Rect {
        let (_, rect) = self
            .font
            .measure_str(self.content(), Some(&self.rule.inner));
        Rect {
            x: rect.x(),
            y: rect.y(),
            width: rect.width(),
            height: rect.height(),
        }
    }

    #[inline]
    ///Gets the bounds of this Text on the GUI. Note that even a text contains a Rect to determine where it should stop rendering, what this function does is to retrieve it, even though the text underflows it.
    ///If you are trying to get the bounds of the text itself, without caring about where it should be drawn, use `text_bounds` instead
    pub fn bounds(&self) -> Rect {
        Rect {
            x: self.position.x,
            y: self.position.y,
            width: self.size.x,
            height: self.size.y,
        }
    }

    ///Applies the given style to this square
    #[inline]
    pub fn apply_style(&mut self, style: &dyn Style) {
        self.rule.apply_style(style);
        self.rule
            .inner
            .set_color4f(vec4f32_to_color(&style.color()), None);
    }

    #[inline]
    pub fn with_style(mut self, style: &dyn Style) -> Self {
        self.rule.apply_style(style);
        self.rule
            .inner
            .set_color4f(vec4f32_to_color(&style.color()), None);
        self
    }
}
