use std::ops::{Deref, DerefMut, Range};

use nalgebra::Vector2;

use crate::{
    components::Text,
    elements::CandySquare,
    helpers::{char_size_backwards, char_size_init},
    renderer::twod::BiDimensionalPainter,
    ui::component::Component,
};

#[derive(Debug)]
///A Input that can be eitger Text, Numeric or Password. Text input accepts any kind of input. Numeric will only accept numbers and
///Password will accept everything, such as Text, but will hide the content written
pub enum Input {
    Text(RawInput),
    Numeric(RawInput),
    Password(RawInput, String), //the second field is the actually value
}

#[derive(Debug)]
pub struct RawInput {
    ///The content to be shown on the screen
    content: Text,
    ///The square this input has got
    rect: CandySquare,
    ///The square of the cursor of this input
    cursor_square: CandySquare,
    ///The index of the cursor on `content`
    cursor: usize,
}

impl Input {
    ///Creates a new Input that accepts Strings in general as long as they're utf8 with the initial text being the provided `content`
    pub fn new(content: Text) -> Self {
        Self::Text(RawInput::new(content))
    }

    ///Creates a new Input that accepts Strings in general as long as they're utf8 with the initial text being the provided `content`
    pub fn new_numeric(content: Text) -> Self {
        Self::Numeric(RawInput::new(content))
    }

    ///Creates a new Input that accepts Strings in general as long as they're utf8 with the initial text being the provided `content`
    pub fn new_password(mut content: Text) -> Self {
        let text = content.content().to_string();
        {
            let len = content.content().chars().count();
            let text_ref = content.content_mut();
            text_ref.clear();
            text_ref.push_str(str::from_utf8(&vec![b'*'; len]).unwrap());
        }

        Self::Password(RawInput::new(content), text)
    }

    #[inline]
    ///Retrieves the raw input which contains the data and logic
    fn raw(&self) -> &RawInput {
        match self {
            Self::Text(t) => t,
            Self::Numeric(t) => t,
            Self::Password(t, _) => t,
        }
    }
    #[inline]
    ///Retrieves the raw input which contains the data and logic
    fn raw_mut(&mut self) -> &mut RawInput {
        match self {
            Self::Text(t) => t,
            Self::Numeric(t) => t,
            Self::Password(t, _) => t,
        }
    }

    ///Writes the given `ch` at the current cursor position and moves it to after the current char
    pub fn write(&mut self, ch: char) {
        match self {
            Self::Text(t) => {
                t.content.content_mut().insert(t.cursor, ch);
                t.cursor += ch.len_utf8();
                t.update_cursor();
            }
            Self::Numeric(t) if ch.is_numeric() => {
                t.content.content_mut().insert(t.cursor, ch);
                t.cursor += ch.len_utf8();
                t.update_cursor();
            }
            Self::Password(t, content) => {
                t.content.content_mut().insert(t.cursor, '*');
                content.insert(t.cursor, ch);
                t.update_cursor();
            }
            _ => {}
        }
    }

    ///Writes the given `str` at the current cursor position and moves it to the end of the inserted content
    pub fn write_str(&mut self, str: &str) {
        match self {
            Self::Text(t) => {
                t.content.content_mut().insert_str(t.cursor, str);
                t.cursor += str.len();
                t.update_cursor();
            }
            Self::Numeric(t) if str.parse::<f32>().is_ok() => {
                t.content.content_mut().insert_str(t.cursor, str);
                t.cursor += str.len();
                t.update_cursor();
            }
            Self::Password(t, content) => {
                let len = str.len();
                let vec = vec![b'*'; len];
                t.content
                    .content_mut()
                    .insert_str(t.cursor, unsafe { str::from_utf8_unchecked(&vec) });
                content.insert_str(t.cursor, str);
                t.cursor += len;
                t.update_cursor();
            }
            _ => {}
        }
    }
}

impl Deref for Input {
    type Target = RawInput;
    fn deref(&self) -> &Self::Target {
        self.raw()
    }
}
impl DerefMut for Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.raw_mut()
    }
}

impl RawInput {
    ///Creates a new Input which will accept any kind of char as long as it's utf8 valid
    pub fn new(content: Text) -> Self {
        Self {
            cursor_square: CandySquare::default(),
            rect: CandySquare::default(),
            cursor: content.content().len(),
            content,
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
        fn char_to_byte_index(s: &str, char_idx: usize) -> (usize, usize) {
            s.char_indices()
                .nth(char_idx)
                .map(|(i, c)| (i, c.len_utf8()))
                .unwrap_or((s.len(), 1)) // if char_idx == len, return end of string
        }
        let visible = self.visible_chars();
        let start_byte = char_to_byte_index(self.content(), visible.start).0;
        let (indice, len) = char_to_byte_index(self.content(), self.cursor);
        self.cursor_square.position_mut().x = {
            self.content
                .font()
                .width_for(&self.content()[start_byte..indice - (len - 1)])
        };
        //totally arbitary numbers. seriosuly, i just tested until i found that it was 0.75
        self.cursor_square.position_mut().y =
            self.content.position().y - self.content.font().size() * 0.75;
    }

    ///Moves the cursor to the right by the given `amount` of chars updates it's GUI
    pub fn move_right(&mut self, mut amount: usize) {
        if amount == 0 || self.is_cursor_at_end() {
            return;
        }
        while amount > 0 {
            let char_size = char_size_init(self.content().as_bytes()[self.cursor - 1]) as usize;
            self.cursor += char_size;
            amount -= 1;
        }
        self.cursor = self.cursor.min(self.content().len());
        self.update_cursor();
    }

    ///Moves the cursor to the left by the given `amount` of chars and updates it's GUI
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

    ///Gets a range containing the indices of all the visible chars on this input based on the cursor position
    pub fn visible_chars(&self) -> Range<usize> {
        let size = self.content.font().size();
        let bounds = self.content.bounds();
        let content_len = self.content().chars().count();
        let half = (bounds.width * size.recip()) as usize;
        let approx = (half * 2).min(content_len);
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
    fn render(&self, renderer: &mut dyn BiDimensionalPainter) {
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
    }
    fn position(&self) -> Vector2<f32> {
        self.content.position()
    }
    fn position_mut(&mut self) -> &mut Vector2<f32> {
        self.content.position_mut()
    }
}
