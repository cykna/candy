use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use nalgebra::Vector4;

use crate::{
    elements::{CandySquare, DrawRule},
    renderer::{CandyRenderer, twod::BiDimensionalPainter},
    ui::component::Component,
};

#[derive(Debug)]
pub struct SolidBox<R: CandyRenderer> {
    square: CandySquare,
    phantom: PhantomData<R>,
}

impl<R: CandyRenderer> Component<R> for SolidBox<R> {
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.square.resize(rect);
    }
    fn render(&self, renderer: &mut <R as CandyRenderer>::TwoD) {
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

impl<R: CandyRenderer> SolidBox<R> {
    pub fn new(color: &Vector4<f32>) -> Self {
        let mut this = Self {
            square: CandySquare::default(),
            phantom: PhantomData,
        };
        this.set_color(color);
        this
    }
}

impl<R: CandyRenderer> Deref for SolidBox<R> {
    type Target = DrawRule;
    fn deref(&self) -> &Self::Target {
        &self.square.rule
    }
}

impl<R: CandyRenderer> DerefMut for SolidBox<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.square.rule
    }
}
