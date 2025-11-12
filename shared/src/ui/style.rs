use nalgebra::{Vector2, Vector4};

use crate::{Effect, NoEffect};

pub trait Style: std::fmt::Debug {
    ///Retrieves the color of this Style
    fn color(&self) -> Vector4<f32> {
        Vector4::new(0.0, 0.0, 0.0, 1.0)
    }

    ///Retrieves the background color of this style. Generally this is the color itself when used on boxes and the `color` is the color when used on texts
    fn background_color(&self) -> Vector4<f32> {
        Vector4::new(1.0, 1.0, 1.0, 1.0)
    }

    ///Retrieves the effects of this Style
    fn effect(&self) -> Box<dyn Effect> {
        Box::new(NoEffect)
    }

    ///The border color of the element, if it's got some. The element must be a square under the hood to this be applied
    fn border_color(&self) -> Vector4<f32> {
        Vector4::new(1.0, 1.0, 1.0, 1.0)
    }

    ///The border radius of the element, if it's got some. The element must be a square under the hood to this be applied
    fn border_radius(&self) -> Vector2<f32> {
        Vector2::new(8.0, 8.0)
    }

    ///The border width of the element, if it's got some. The element must be a square under the hood to this be applied
    fn border_width(&self) -> f32 {
        0.0
    }
}

#[derive(Debug)]
///Default style for anything. It's defined as:
///Border Radius: 8px 8px
///Effect: None
///Color: Black
///BorderWidth: 0
///BorderColor: White
pub struct DefaultStyle;
impl Style for DefaultStyle {}
