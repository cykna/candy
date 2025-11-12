use std::ops::{Deref, DerefMut};

use candy_renderers::{
    BiDimensionalPainter,
    primitives::{CandySquare, DrawRule},
};
use candy_shared_types::{Rect, Style};
use nalgebra::Vector4;

use crate::ui::component::Component;

#[derive(Debug)]
pub struct SolidBox {
    square: CandySquare,
}

impl Component for SolidBox {
    fn resize(&mut self, rect: Rect) {
        self.square.resize(rect);
    }

    fn render(&self, renderer: &mut dyn BiDimensionalPainter) {
        renderer.square(&self.square);
    }
    fn apply_style(&mut self, style: &dyn Style) {
        self.square.apply_style(style);
    }
    fn position(&self) -> nalgebra::Vector2<f32> {
        *self.square.position()
    }
    fn position_mut(&mut self) -> &mut nalgebra::Vector2<f32> {
        self.square.position_mut()
    }
    fn apply_offset(&mut self, offset: nalgebra::Vector2<f32>) {
        *self.square.position_mut() += offset;
    }
}

impl SolidBox {
    pub fn new(color: &Vector4<f32>) -> Self {
        let mut this = Self {
            square: CandySquare::default(),
        };
        this.set_color(color);
        this
    }
}

impl Deref for SolidBox {
    type Target = DrawRule;
    fn deref(&self) -> &Self::Target {
        &self.square.rule
    }
}

impl DerefMut for SolidBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.square.rule
    }
}
