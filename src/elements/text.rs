use crate::text::font::CandyFont;
use nalgebra::{Vector2, Vector4};

///The alignment of a Text. Wheater it will be positioned on the Left, Right, or Center of it's parent
///Default is Center
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum TextAlignment {
    #[default]
    Center,
    Right,
    Left,
}

///A handler that contains on how to draw an specific text
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

    ///Gets the alignment of this text
    #[inline]
    pub fn alignment(&self) -> TextAlignment {
        self.align
    }

    ///Gets the bounds of this Text. XY are the position while ZW are width and height
    #[inline]
    pub fn bounds(&self) -> Vector4<f32> {
        let mut rec = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let mut glyphs = Vec::with_capacity(self.content().len());
        self.font.str_to_glyphs(self.content(), &mut glyphs);
        self.font.get_widths(&glyphs, rec.as_mut_slice());
        rec
    }
}
