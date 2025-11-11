use std::ops::{Deref, DerefMut};

use nalgebra::Vector2;
use skia_safe::{Data, Image};

use crate::ui::component::RendererImage;

use super::CandySquare;

/// A handler for Images on Candy. This is now shown due to rust limitations with dyn, but this is dependent of CandyImgConstructor
pub trait TwodCandyImg: Sized + std::fmt::Debug {
    fn from_source<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self>;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

#[derive(Debug)]
///An Image used to be drawn on by the painter P
pub struct CandyImage {
    inner: RendererImage,
    square: CandySquare,
}
impl CandyImage {
    ///Tries to create a new image from `source`. If no `square` is given, the it will use the normal
    ///resolution of the image positioned at (0,0)
    pub fn from_source<Ph: AsRef<std::path::Path>>(
        source: Ph,
        square: Option<CandySquare>,
    ) -> std::io::Result<Self> {
        let inner = RendererImage::from_source(source)?;
        Ok(Self {
            square: square.unwrap_or(CandySquare::new(
                Vector2::zeros(),
                Vector2::new(inner.width() as f32, inner.height() as f32),
            )),
            inner,
        })
    }
}
impl CandyImage {
    #[inline]
    ///Creates a new CandyImage with the given `img` and defining it's square info to be the given `square`
    ///Note: The color of the `square` will be used to multiply the colors of the texture when drawn
    pub fn new(img: RendererImage, square: CandySquare) -> Self {
        Self { inner: img, square }
    }

    ///Gets the position of this Image
    #[inline]
    pub fn position(&self) -> &Vector2<f32> {
        self.square.position()
    }

    #[inline]
    ///Returns the actual width of the image and not the size it will be drawn
    pub fn real_width(&self) -> i32 {
        self.inner.width() as i32
    }

    #[inline]
    ///Returns the actual height of the image and not the size it will be drawn
    pub fn real_height(&self) -> i32 {
        self.inner.height() as i32
    }

    #[inline]
    ///Returns the inner image handle
    pub fn image_handler(&self) -> &RendererImage {
        &self.inner
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

impl TwodCandyImg for skia_safe::Image {
    fn from_source<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Self> {
        let data = std::fs::read(path)?;
        let inner = Image::from_encoded(Data::new_copy(&data)).unwrap();

        Ok(inner)
    }
    #[inline]
    fn width(&self) -> u32 {
        self.width() as u32
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height() as u32
    }
}
