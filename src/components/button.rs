use nalgebra::Vector2;
use winit::event::MouseButton;

use crate::{
    components::Text, elements::CandySquare, renderer::twod::BiDimensionalPainter,
    ui::component::Component,
};

pub struct Button<Msg> {
    text: Text,
    rect: CandySquare,
    func: Box<dyn Fn(Vector2<f32>, MouseButton) -> Msg>,
}

impl<Msg> Component for Button<Msg> {
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        let bounds = self.text.bounds().center();
        let center = rect.center();
        self.text.position_mut().x = center.x - bounds.x;
        self.text.position_mut().y = center.y - bounds.y;
        *self.rect.position_mut() = Vector2::new(rect.x, rect.y);
        *self.rect.size_mut() = Vector2::new(rect.width, rect.height);
    }
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        renderer.square(&self.rect);
        self.text.render(renderer);
    }
    fn apply_style<S>(&mut self, style: &S)
    where
        S: crate::ui::styling::style::Style,
    {
        self.text.apply_style(style);
        self.rect.apply_style(style);
    }
}

impl<Msg> Button<Msg> {
    pub fn new<F>(text: Text, f: F) -> Self
    where
        F: Fn(Vector2<f32>, MouseButton) -> Msg + 'static,
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
}
