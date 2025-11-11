use std::ops::{Deref, DerefMut};

use nalgebra::Vector2;
use winit::event::MouseButton;

use crate::{
    components::Text,
    elements::CandySquare,
    helpers::center,
    renderer::twod::BiDimensionalPainter,
    ui::{component::Component, styling::style::Style},
};

pub struct Button<'a, Msg> {
    text: Text,
    rect: CandySquare,
    func: Box<dyn Fn(Vector2<f32>, MouseButton) -> Msg + 'a>,
}

impl<'a, Msg> Component for Button<'a, Msg> {
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        *self.text.position_mut() = center(&self.text.text_bounds(), &rect);
        *self.text.size_mut() = Vector2::new(rect.width, rect.y);
        *self.rect.position_mut() = Vector2::new(rect.x, rect.y);
        *self.rect.size_mut() = Vector2::new(rect.width, rect.height);
    }
    fn render(&self, renderer: &mut dyn BiDimensionalPainter) {
        renderer.square(&self.rect);
        self.text.render(renderer);
    }
    fn apply_style(&mut self, style: &dyn Style) {
        self.text.apply_style(style);
        self.rect.apply_style(style);
    }
    fn position(&self) -> Vector2<f32> {
        *self.rect.position()
    }
    fn position_mut(&mut self) -> &mut Vector2<f32> {
        self.rect.position_mut()
    }
    fn apply_offset(&mut self, offset: Vector2<f32>) {
        self.text.apply_offset(offset);
        *self.rect.position_mut() += offset;
    }
}

impl<'a, Msg> Button<'a, Msg> {
    ///Creates a new Button with the given `text` centered and executing `f` when clicked
    pub fn new<F>(text: Text, f: F) -> Self
    where
        F: (Fn(Vector2<f32>, MouseButton) -> Msg) + 'a,
    {
        Self {
            text,
            rect: CandySquare::new(Vector2::zeros(), Vector2::new(50.0, 50.0)),
            func: Box::new(f),
        }
    }
    #[inline]
    ///Tries to execute the function of this button. Returns some if it did execute, otherwise returns None
    ///Being executed means this button was clicked.
    pub fn try_exec(&self, pos: Vector2<f32>, btn: MouseButton) -> Option<Msg> {
        if self.rect.bounds().contains(pos) {
            Some((self.func)(pos, btn))
        } else {
            None
        }
    }

    #[inline]
    ///Forces the execution of the function of this button even though it was not actually clicked
    pub fn force_execution(&self, pos: Vector2<f32>, btn: MouseButton) -> Msg {
        (self.func)(pos, btn)
    }

    #[inline]
    ///Retrieves the content of the text of this button
    pub fn content(&self) -> &str {
        self.text.content()
    }
    #[inline]
    ///Retrieves the content of the text of this button
    pub fn content_mut(&mut self) -> &mut String {
        self.text.content_mut()
    }

    #[inline]
    ///Applies the provided `style` and returns itself. Mainly used for chaining.
    pub fn with_style(mut self, style: &dyn Style) -> Self {
        self.apply_style(style);
        self
    }
}

impl<'a, Msg> Deref for Button<'a, Msg> {
    type Target = CandySquare;
    fn deref(&self) -> &Self::Target {
        &self.rect
    }
}

impl<'a, Msg> DerefMut for Button<'a, Msg> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rect
    }
}

impl<'a, Msg> std::fmt::Debug for Button<'a, Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("text", &self.text.inner())
            .field("rect", &self.rect)
            .field("func", &"fn internal();")
            .finish()
    }
}
