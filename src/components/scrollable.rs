use std::ops::{Deref, DerefMut};

use nalgebra::Vector2;

use crate::{
    components::container::Container,
    helpers::rect::Rect,
    ui::{
        component::Component,
        styling::{
            layout::{DefinitionRect, Direction, Layout, Size},
            style::Style,
        },
    },
};

pub struct Scrollable {
    direction: Direction,
    container: Container,
    scrollbar: Container,
    layout: Layout,
    old_cursor: Vector2<f32>,
    offset: f32,
    is_dragging: bool,
}

pub struct ScrollableConfig {
    pub scroll_bar_width: f32,
    pub direction: Direction,
    pub layout: Layout,
}

impl Scrollable {
    ///Generates a ScrollBar for a Scrollable
    pub fn scroll_bar() -> Container {
        let mut out = Container::new(Layout::vertical(), false);
        out.add_child(
            Container::new(Layout::vertical(), false),
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
            is_dragging: false,
        }
    }

    pub fn is_dragging(&mut self) -> bool {
        self.is_dragging
    }

    pub fn scrollbar(&self) -> &Container {
        &self.scrollbar
    }

    ///Applies the given `style` to the scrollbar
    pub fn apply_style_scrollbar(&mut self, style: &dyn Style) {
        self.scrollbar.apply_style(style);
    }

    ///Checks if the given `pos` is inside the scrollbar, if so, treat it as a click and toggles if it's scrolling or not
    pub fn on_mouse(&mut self, pos: Vector2<f32>) {
        if self.scrollbar.bounds().contains(pos) {
            self.is_dragging = !self.is_dragging;
            self.old_cursor = pos;
        }
    }

    ///Modifies the offset based on the given `pos` and assings the new cursor position to be the `pos` so values can be tracked correctly
    pub fn on_cursor(&mut self, pos: Vector2<f32>) {
        self.offset = match self.direction {
            Direction::Vertical => pos.y - self.old_cursor.y,
            Direction::Horizontal => pos.x - self.old_cursor.x,
        };
        self.old_cursor = pos;
    }

    ///Updates the positions of all the elements inside this scrollable based on the scroll offset
    pub fn update_positions(&mut self) {
        for child in self.container.children_mut() {
            child.position_mut().y += self.offset;
        }
    }
}

impl Deref for Scrollable {
    type Target = Container;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}
impl DerefMut for Scrollable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.container
    }
}

impl Component for Scrollable {
    fn resize(&mut self, rect: Rect) {
        let rects = self.layout.calculate(rect, true);
        self.scrollbar.resize(rects[0].clone());
        self.container.resize(rects[1].clone());
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
}
