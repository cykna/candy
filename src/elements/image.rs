use std::ops::{Deref, DerefMut};

use nalgebra::{Vector2, Vector4};
use skia_safe::{Data, Image, canvas::SrcRectConstraint};

use crate::renderer::twod::CandyImg;

use super::CandySquare;

pub struct CandyImage {
    inner: Box<dyn CandyImg>,
    square: CandySquare,
}

impl CandyImage {
    ///Tries to create a new image from `source`. If no `square` is given, the it will use the normal
    ///resolution of the image positioned at (0,0)
    pub fn from_source<P: AsRef<std::path::Path>>(
        source: P,
        square: Option<CandySquare>,
    ) -> std::io::Result<Self> {
        let data = std::fs::read(source)?;
        let inner = Image::from_encoded(Data::new_copy(&data)).unwrap();

        Ok(Self {
            square: square.unwrap_or(CandySquare::new(
                Vector2::zeros(),
                Vector2::new(inner.width() as f32, inner.height() as f32),
                Vector4::zeros(),
                None,
                None,
            )),
            inner: Box::new(inner),
        })
    }
    #[inline]
    ///Creates a new CandyImage with the given `img` and defining it's square info to be the given `square`
    ///Note: The color of the `square` will be used to multiply the colors of the texture when drawn
    pub fn new(img: impl CandyImg + 'static, square: CandySquare) -> Self {
        Self {
            inner: Box::new(img),
            square,
        }
    }

    #[inline]
    pub fn position(&self) -> &Vector2<f32> {
        self.square.position()
    }

    #[inline]
    ///Returns the actual width of the image and not the size it will be drawn
    pub fn real_width(&self) -> u32 {
        self.inner.width()
    }

    #[inline]
    ///Returns the actual height of the image and not the size it will be drawn
    pub fn real_height(&self) -> u32 {
        self.inner.height()
    }

    #[inline]
    ///Returns the inner image handle
    pub fn image_handler(&self) -> &dyn CandyImg {
        &*self.inner
    }
}

impl Deref for CandyImage {
    type Target = CandySquare;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.square
    }
}
impl DerefMut for CandyImage {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.square
    }
}
