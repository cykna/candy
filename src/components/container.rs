use std::ops::{Deref, DerefMut};

use crate::{
    elements::CandySquare,
    renderer::twod::BiDimensionalPainter,
    ui::{component::Component, styling::style::Style},
};

pub struct Container {
    square: CandySquare,
    children: Vec<Box<dyn Component>>,
}

impl Component for Container {
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        if self.square.rule.get_color().w == 0.0 && self.square.rule.border_color.w == 0.0 {
            return;
        }
        renderer.square(&self.square);
    }
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.square.position_mut().x = rect.x;
        self.square.position_mut().y = rect.y;
        self.square.size_mut().x = rect.width;
        self.square.size_mut().y = rect.height;
    }
    fn apply_style(&mut self, style: &dyn Style) {
        self.square.apply_style(style);
    }
}

impl Container {
    pub fn new() -> Self {
        Self {
            square: CandySquare::default(),
            children: Vec::new(),
        }
    }
}

impl Deref for Container {
    type Target = CandySquare;
    fn deref(&self) -> &Self::Target {
        &self.square
    }
}

impl DerefMut for Container {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.square
    }
}
