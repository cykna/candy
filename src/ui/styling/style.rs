use nalgebra::{Vector2, Vector4};

use crate::{
    components::Text,
    ui::{
        component::Component,
        styling::fx::{Effect, NoEffect},
    },
};

pub trait StyleProvider<C>
where
    C: Component,
{
    fn retrieve_style_for(&self, _: &C) -> impl Style {
        DefaultStyle
    }
    fn style_for(_: &C) -> impl Style {
        DefaultStyle
    }
}

pub trait Style {
    ///Retrieves the color of this Style
    fn color(&self) -> Vector4<f32> {
        Vector4::new(0.0, 0.0, 0.0, 1.0)
    }

    fn background_color(&self) -> Vector4<f32> {
        Vector4::new(1.0, 1.0, 1.0, 1.0)
    }

    ///Retrieves the effects of this Style
    fn effect(&self) -> Box<dyn Effect> {
        Box::new(NoEffect)
    }

    fn border_color(&self) -> Vector4<f32> {
        Vector4::new(1.0, 1.0, 1.0, 1.0)
    }

    fn border_radius(&self) -> Vector2<f32> {
        Vector2::new(8.0, 8.0)
    }

    fn border_width(&self) -> f32 {
        0.0
    }
}

pub struct DefaultProvider;
impl StyleProvider<Text> for DefaultProvider {}
pub struct DefaultStyle;
impl Style for DefaultStyle {}
