use std::ops::{Deref, DerefMut, Div, Mul};

use nalgebra::{Vector2, Vector4};

use crate::{
    components::{SolidBox, container::Container},
    helpers::rect::Rect,
    ui::{
        component::Component,
        styling::{
            layout::{DefinitionRect, Direction, Layout, Size},
            style::Style,
        },
    },
};

pub struct Scrollable<C: Component> {
    direction: Direction,
    container: Container<C>,
    scrollbar: Container<SolidBox>,
    layout: Layout,
    old_cursor: Vector2<f32>,
    offset: f32,
    accum_offset: f32,
    is_dragging: bool,
    limit: f32,
}

pub struct ScrollableConfig {
    pub scroll_bar_width: f32,
    pub direction: Direction,
    pub layout: Layout,
}

impl<C: Component> Scrollable<C> {
    ///Generates a ScrollBar for a Scrollable
    pub fn scroll_bar() -> Container<SolidBox> {
        let mut out = Container::new(Layout::vertical(), false);
        out.add_child(
            SolidBox::new(&Vector4::new(0.0, 0.0, 0.0, 1.0)),
            DefinitionRect {
                x: Size::Length(0.0),
                y: Size::Length(0.0),
                width: Size::Percent(1.0),
                height: Size::Length(0.5),
            },
        );
        out
    }

    pub fn new(config: ScrollableConfig) -> Self {
        let mut layout = Layout::new(match config.direction {
            Direction::Vertical => Direction::Horizontal,
            Direction::Horizontal => Direction::Vertical,
        }); //if its vertical, the scrollbar and the content are set side by side, else scrollbar
        //on top
        match config.direction {
            Direction::Vertical => layout
                .with_definition(DefinitionRect {
                    x: Size::Length(0.0),
                    y: Size::Length(0.0),
                    width: Size::Length(config.scroll_bar_width),
                    height: Size::Percent(1.0),
                })
                .with_definition(DefinitionRect {
                    x: Size::Length(0.0),
                    y: Size::Length(0.0),
                    width: Size::Percent(1.0),
                    height: Size::Percent(1.0),
                }),
            Direction::Horizontal => layout
                .with_definition(DefinitionRect {
                    x: Size::Length(0.0),
                    y: Size::Length(0.0),
                    width: Size::Percent(1.0),
                    height: Size::Length(config.scroll_bar_width),
                })
                .with_definition(DefinitionRect {
                    x: Size::Length(0.0),
                    y: Size::Length(0.0),
                    width: Size::Percent(1.0),
                    height: Size::Percent(1.0),
                }),
        };
        let container = Container::new(config.layout, true);

        Self {
            layout,
            container,
            scrollbar: Self::scroll_bar(),
            direction: config.direction,
            old_cursor: Vector2::zeros(),
            offset: 0.0,
            accum_offset: 0.0,
            is_dragging: false,
            limit: 0.0,
        }
    }

    #[inline]
    ///Returns weather this scrollable is dragging or not
    pub fn is_dragging(&mut self) -> bool {
        self.is_dragging
    }

    #[inline]
    ///Returns the element of the scrollbar
    pub fn scrollbar(&self) -> &Container<SolidBox> {
        &self.scrollbar
    }

    #[inline]
    ///Applies the given `style` to the scrollbar element
    pub fn apply_style_scrollbar(&mut self, style: &dyn Style) {
        self.scrollbar.apply_style(style);
    }

    ///Checks if the given `pos` is inside the scrollbar, if so, treat it as a click and toggles if it's scrolling or not
    pub fn on_mouse_click(&mut self, pos: Vector2<f32>) {
        if self.scrollbar.bounds().contains(pos) {
            self.is_dragging = !self.is_dragging;
            if self.is_dragging {
                self.old_cursor = pos;
            }
        }
    }

    #[inline]
    ///Directly applies the given `offset` on the offset of this scrollable and updates the new positions
    pub fn drag_offset(&mut self, offset: Vector2<f32>) {
        self.offset = match self.direction {
            Direction::Vertical => offset.y,
            Direction::Horizontal => offset.x,
        };
        let sum = self.accum_offset + self.offset;
        if sum < self.limit || sum > 0.0 {
            return;
        }
        self.accum_offset = sum;

        self.update_positions();
    }

    #[inline]
    ///Modifies the position of the elements based on the new `pos` provided. If `scrollable.is_dragging() == false`, then this doesn't do anything
    pub fn drag(&mut self, pos: Vector2<f32>) {
        if self.is_dragging {
            self.offset = match self.direction {
                Direction::Vertical => pos.y - self.old_cursor.y,
                Direction::Horizontal => pos.x - self.old_cursor.x,
            };
            self.old_cursor = pos;
            let sum = self.accum_offset + self.offset;
            if sum < self.limit || sum > 0.0 {
                return;
            }
            self.accum_offset = sum;

            self.update_positions();
        }
    }

    ///Updates the positions of the inner elements based on the accumulated offset.
    fn update_positions_accum(&mut self) {
        for child in self.container.children_mut() {
            child.apply_offset(Vector2::new(0.0, self.accum_offset));
        }
    }

    ///Updates the positions of all the elements inside this scrollable based on the scroll offset
    fn update_positions(&mut self) {
        for child in self.container.children_mut() {
            child.apply_offset(Vector2::new(0.0, self.offset));
        }
    }
}

impl<C: Component> Deref for Scrollable<C> {
    type Target = Container<C>;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
impl<C: Component> DerefMut for Scrollable<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.container
    }
}

impl<C: Component> Component for Scrollable<C> {
    fn resize(&mut self, rect: Rect) {
        let height = rect.height;
        let rects = self.layout.calculate(rect, true);
        self.scrollbar.resize(rects[0].clone());
        self.container.resize(rects[1].clone());
        let calc_height = self
            .container
            .layout
            .calculate_height(rects[1].clone(), true);
        if calc_height > height {
            self.limit = height - calc_height;
        }
        self.update_positions_accum();
    }
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        self.container.render(renderer);
        self.scrollbar.render(renderer);
    }

    fn apply_style(&mut self, style: &dyn Style) {
        self.container.apply_style(style);
    }

    fn position(&self) -> Vector2<f32> {
        self.container.position()
    }

    fn position_mut(&mut self) -> &mut Vector2<f32> {
        self.container.position_mut()
    }

    fn apply_offset(&mut self, offset: Vector2<f32>) {
        *self.container.position_mut() += offset;
        *self.scrollbar.position_mut() += offset;
    }
}
