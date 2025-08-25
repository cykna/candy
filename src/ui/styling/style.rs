use nalgebra::{Vector2, Vector4};

use crate::ui::styling::fx::{Effect, NoEffect};

pub trait Style {
    ///Retrieves the color of this Style
    fn color(&self) -> Vector4<f32> {
        Vector4::new(1.0, 1.0, 1.0, 1.0)
    }

    ///Retrieves the effects of this Style
    fn effect(&self) -> impl Effect {
        NoEffect
    }

    fn border_color(&self) -> Vector4<f32> {
        Vector4::zeros()
    }

    fn border_radius(&self) -> Vector2<f32> {
        Vector2::zeros()
    }

    fn border_width(&self) -> f32 {
        0.0
    }
}

pub struct DefaultStyle;
impl Style for DefaultStyle {}
