use candy_shared_types::{Rect, Style};
use nalgebra::Vector2;

use crate::primitives::DrawRule;

///A handler that contains information about how a square should be drawn.
#[derive(Debug, Default)]
pub struct CandySquare {
    position: Vector2<f32>,
    size: Vector2<f32>,
    pub rule: DrawRule,
}

impl CandySquare {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Self {
        let rule = DrawRule::new();
        Self {
            position,
            size,
            rule,
        }
    }

    ///Gets the position of this square
    ///Obs: As this gets mutable, this code assumes the data will be changed, so, this is marked as dirty
    pub fn position_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.position
    }

    ///Gets the actual size of this square
    ///Obs: As this gets mutable, this code assumes the data will be changed, so, this is marked as dirty    ///
    pub fn size_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.size
    }

    ///Gets the position of this square
    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }

    ///Gets the actual size of this square
    pub fn size(&self) -> &Vector2<f32> {
        &self.size
    }

    pub fn resize(&mut self, rect: Rect) {
        self.size.x = rect.width;
        self.size.y = rect.height;
        self.position.x = rect.x;
        self.position.y = rect.y;
    }

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
    }

    #[inline]
    pub fn with_style(mut self, style: &dyn Style) -> Self {
        self.rule.apply_style(style);
        self
    }
}
