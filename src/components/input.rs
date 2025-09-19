use std::ops::Range;

use nalgebra::Vector2;

use crate::{
    components::Text,
    elements::CandySquare,
    helpers::{char_size_backwards, char_size_init},
    renderer::twod::BiDimensionalPainter,
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

    ///Retrieves weather the cursor is at the end of the content of this Input
    #[inline]
    pub fn is_cursor_at_end(&self) -> bool {
        self.cursor == self.content().len()
    }

    #[inline]
    ///Retrieves the amount of chars that are visible within the bounds of this text
    pub fn visible_quantity(&self) -> f32 {
        self.content.text_width() / self.content.font().size()
    }

    ///Updates the cursor position on the GUI. In fact, if it did change the position, sets the cursor square to be there
    pub fn update_cursor(&mut self) {
        fn char_to_byte_index(s: &str, char_idx: usize) -> usize {
            s.char_indices()
                .nth(char_idx)
                .map(|(i, _)| i)
                .unwrap_or(s.len()) // if char_idx == len, return end of string
        }
        let visible = self.visible_chars();
        let start_byte = char_to_byte_index(self.content(), visible.start);
        let cursor_byte = char_to_byte_index(self.content(), self.cursor);
        self.cursor_square.position_mut().x = {
            self.content
                .font()
                .width_for(&self.content()[start_byte..cursor_byte])
                + self.content.font().size() * 0.25
        };
        //totally arbitary numbers. seriosuly, i just tested until i found that it was 0.75
        self.cursor_square.position_mut().y =
            self.content.position().y - self.content.font().size() * 0.75;
    }

    ///Moves the cursor to the right by the given `amount` of chars updates it's GUI
    #[inline]
    pub fn move_right(&mut self, amount: usize) {
        if amount == 0 {
            return;
        }
        self.cursor = (self.cursor + amount).min(self.content().len());
        self.update_cursor();
    }

    ///Moves the cursor to the left by the given `amount` of chars and updates it's GUI
    #[inline]
    pub fn move_left(&mut self, mut amount: usize) {
        if amount == 0 || self.cursor == 0 {
            return;
        }
        while amount > 0 {
            let char_size = char_size_backwards(&self.content().as_bytes(), self.cursor - 1);
            self.cursor -= char_size;
            amount -= 1;
        }

        self.update_cursor();
    }

    #[inline]
    ///Writes the given `ch` at the current cursor position and moves it to after the current char
    pub fn write(&mut self, ch: char) {
        self.content.content_mut().insert(self.cursor, ch);
        self.cursor += ch.len_utf8();
        self.update_cursor();
    }

    #[inline]
    ///Writes the given `str` at the current cursor position and moves it to the end of the inserted content
    pub fn write_str(&mut self, str: &str) {
        self.content.content_mut().insert_str(self.cursor, str);
        self.cursor += str.len();
        self.update_cursor();
    }

    ///Gets a range containing the indices of all the visible chars on this input based on the cursor position
    pub fn visible_chars(&self) -> Range<usize> {
        let size = self.content.font().size();
        let bounds = self.content.bounds();
        let content_len = self.content().chars().count();
        let half = (bounds.width * size.recip()) as usize;
        let approx = (half << 1).min(content_len);
        if content_len <= approx {
            0..content_len
        } else if self.cursor <= half {
            0..approx
        } else if self.cursor >= content_len - half {
            content_len - approx..content_len
        } else {
            self.cursor - half..self.cursor + half
        }
    }
}

impl Component for Input {
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.rect.resize(rect.clone());

        let content_bounds = self.content.text_bounds();
        //y center
        self.content.position_mut().y = rect.center().y + content_bounds.height * 0.5;
        self.content.position_mut().x = rect.x;
        *self.content.size_mut() = Vector2::new(rect.width, rect.height);

        self.update_cursor();
        self.cursor_square.size_mut().y = content_bounds.height + 2.0;
        self.cursor_square.size_mut().x = 1.0;
    }
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        renderer.square(&self.rect);
        {
            let visible = self.visible_chars();
            let mut char_indices: Vec<usize> =
                self.content().char_indices().map(|(i, _)| i).collect();
            char_indices.push(self.content().len()); // safe end index

            let start_byte = char_indices[visible.start];
            let end_byte = char_indices[visible.end];

            renderer.text_sliced(&self.content, start_byte..end_byte);
        }
        renderer.square(&self.cursor_square);
    }
    fn apply_style(&mut self, style: &dyn crate::ui::styling::style::Style) {
        self.rect.apply_style(style);
        self.content.apply_style(style);
        //self.cursor_square.apply_style(style);
    }
}
