use std::rc::Rc;

use nalgebra::Vector2;

use crate::{
    components::container::Container,
    ui::{
        component::Component,
        styling::{layout::Layout, style::Style},
    },
};

pub struct Toggle {
    checked: bool,
    square: Container,
    unchecked_style: Box<dyn Style>,
    checked_style: Box<dyn Style>,
}

impl Toggle {
    pub fn new<U, C>(unchecked: U, checked_style: C) -> Self
    where
        U: Style + 'static,
        C: Style + 'static,
    {
        Self {
            checked: false,
            square: Container::new(Layout::vertical(), false),
            unchecked_style: Box::new(unchecked),
            checked_style: Box::new(checked_style),
        }
    }

    #[inline]
    ///Returns weather clicking on the provided `position` would toggle
    ///Being executed means this button was clicked.
    pub fn would_toggle(&self, pos: Vector2<f32>) -> bool {
        self.square.bounds().contains(pos)
    }

    ///Tries to togle this Toggle with the provided `pos`. Returns Some(old_value) if it did toggle, else returns None.
    ///A side effect of this is to apply the checked/unchecked style.
    pub fn toggle(&mut self, pos: Vector2<f32>) -> Option<bool> {
        if self.would_toggle(pos) {
            self.checked = !self.checked;
            let style = &*if self.checked {
                &*self.checked_style
            } else {
                &*self.unchecked_style
            };
            self.square.apply_style(style);

            Some(true)
        } else {
            None
        }
    }

    #[inline]
    ///Returns weather this Toggle is checked or not
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

impl Component for Toggle {
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.square.resize(rect);
    }
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        self.square.render(renderer);
    }
    #[inline]
    fn apply_style(&mut self, style: &dyn crate::ui::styling::style::Style) {
        self.square.apply_style(style);
    }

    fn position(&self) -> Vector2<f32> {
        self.square.position()
    }
    fn position_mut(&mut self) -> &mut Vector2<f32> {
        self.square.position_mut()
    }
}
