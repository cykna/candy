use nalgebra::Vector2;

use crate::{
    components::Text, elements::CandySquare, renderer::twod::BiDimensionalPainter,
    ui::component::Component,
};

pub enum InputType {
    Text,
    Numeric,
    Password,
}

pub struct Input {
    content: Text,
    rect: CandySquare,
    cursor_square: CandySquare,
    cursor: usize,
    kind: InputType,
}

impl Input {
    ///Creates a new Input with InputType == Number, so it won't show the content being written
    pub fn new_password(content: Text) -> Self {
        Self {
            rect: CandySquare::default(),
            cursor_square: CandySquare::default(),
            cursor: content.content().len(),
            content,
            kind: InputType::Password,
        }
    }
    ///Creates a new Input with InputType == Number, so it will only accept numeric values
    pub fn new_numeric(content: Text) -> Self {
        Self {
            rect: CandySquare::default(),
            cursor_square: CandySquare::default(),
            cursor: content.content().len(),
            content,
            kind: InputType::Numeric,
        }
    }
    ///Creates a new Input which will accept any kind of char as long as it's utf8 valid
    pub fn new(content: Text) -> Self {
        Self {
            cursor_square: CandySquare::default(),
            rect: CandySquare::default(),
            cursor: content.content().len(),
            content,
            kind: InputType::Text,
        }
    }

    ///Retrieves the content of this Input
    #[inline]
    pub fn content(&self) -> &str {
        self.content.content()
    }

    #[inline]
    ///Writes the given `ch` at the current cursor position and moves it to after the current char
    pub fn write(&mut self, ch: char) {
        self.content.content_mut().insert(self.cursor, ch);
        self.cursor += ch.len_utf8();
    }

    #[inline]
    ///Writes the given `str` at the current cursor position and moves it to the end of the inserted content
    pub fn write_str(&mut self, str: &str) {
        self.content.content_mut().insert_str(self.cursor, str);
        self.cursor += str.len();
    }
}

impl Component for Input {
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.rect.resize(rect.clone());

        let content_bounds = self.content.bounds();
        //y center
        self.content.position_mut().y = rect.center().y + content_bounds.height * 0.5;
        self.content.position_mut().x = rect.x;

        self.cursor_square.position_mut().x = {
            let percent = self.cursor as f32 * self.content.font().size() * 0.5;
            percent + self.content.position().x
        };
        self.cursor_square.position_mut().y = content_bounds.y - 1.0;
        self.cursor_square.size_mut().y = content_bounds.height + 2.0;
        self.cursor_square.size_mut().x = 1.0;
    }
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        renderer.square(&self.rect);
        renderer.text(&self.content);
        renderer.square(&self.cursor_square);
    }
    fn apply_style(&mut self, style: &dyn crate::ui::styling::style::Style) {
        self.rect.apply_style(style);
        self.content.apply_style(style);
        //self.cursor_square.apply_style(style);
    }
}
