use std::ops::{Deref, DerefMut};

use nalgebra::Vector4;

use crate::{
    elements::{CandySquare, DrawRule},
    renderer::twod::BiDimensionalPainter,
    ui::component::Component,
};

pub struct SolidBox {
    square: CandySquare,
}

impl Component for SolidBox {
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.square.resize(rect);
    }
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        renderer.square(&self.square);
    }
    fn apply_style(&mut self, style: &dyn crate::ui::styling::style::Style) {
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
