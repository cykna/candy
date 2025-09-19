use std::{fs::File, io::BufReader, ops::Deref};

use skia_safe::{Font, FontMgr, Typeface};

#[derive(Debug, Default, Clone)]
pub struct CandyFont {
    font: Font,
}

impl CandyFont {
    pub fn from_file(path: &str, size: f32) -> Option<Self> {
        let f = File::open(path).ok()?;

        let mut reader = BufReader::new(f);

        Some(Self::new(
            Typeface::make_deserialize(&mut reader, Some(FontMgr::new())).unwrap(),
            size,
        ))
    }

    ///Creates a new Font with the given `size` and searching by `font` name
    pub fn new(face: Typeface, size: f32) -> Self {
        let mut s = Self {
            font: Font::new(face, Some(size)),
        };
        s.with_size(size);
        s
    }

    #[inline]
    ///Sets the size of this font to be the given `size`
    pub fn set_size(&mut self, size: f32) {
        self.font.set_size(size);
    }

    #[inline]
    ///Gets the size of this font
    pub fn size(&self) -> f32 {
        self.font.size()
    }

    ///Retrieves the width in pixels that the given content would have if rendered with this font
    pub fn width_for(&self, text: &str) -> f32 {
        self.font.measure_str(text, None).0
    }
}

impl Deref for CandyFont {
    type Target = Font;
    fn deref(&self) -> &Self::Target {
        &self.font
    }
}
