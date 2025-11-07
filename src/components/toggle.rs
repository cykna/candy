use nalgebra::Vector2;

use crate::{
    components::{SolidBox, container::Container},
    renderer::{CandyRenderer, candy::CandyDefaultRenderer},
    ui::{
        component::Component,
        styling::{layout::Layout, style::Style},
    },
};

#[derive(Debug)]
///A component that represents a toggle button
pub struct Toggle<R = CandyDefaultRenderer>
where
    R: CandyRenderer,
{
    checked: bool,
    square: Container<SolidBox<R>, R>,
    unchecked_style: Box<dyn Style>,
    checked_style: Box<dyn Style>,
}

impl<R> Toggle<R>
where
    R: CandyRenderer,
{
    ///Creates a new toggle. The provided `unchecked` style will be applied when this toggle value is false, `checked_style` will be applied when it's true
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
    ///Returns whether clicking on the provided `position` would toggle
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

impl<C> Component<C> for Toggle<C>
where
    C: CandyRenderer,
{
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.square.resize(rect);
    }
    fn render(&self, renderer: &mut C::TwoD) {
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
