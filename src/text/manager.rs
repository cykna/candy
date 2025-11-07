use skia_safe::{FontMgr, FontStyle};

use crate::text::font::CandyFont;

#[derive(Debug, Default)]
pub struct FontManager {
    inner: FontMgr,
}

impl FontManager {
    pub fn new() -> Self {
        Self::default()
    }
    ///Creates a font with the given `name` and `size`. Panics if the font name is not avaible
    pub fn create_font(&self, name: &str, size: f32) -> Option<CandyFont> {
        let Some(typeface) = self.inner.match_family_style(name, FontStyle::default()) else {
            return None;
        };
        Some(CandyFont::new(typeface, size))
    }
    ///Retrieves a vector containing the name of all avaible fonts
    pub fn avaible_fonts(&self) -> Vec<String> {
        self.inner.family_names().collect()
    }
}
