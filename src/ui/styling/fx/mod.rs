use nalgebra::{Vector2, Vector4};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ShadowEffect {
    pub color: Vector4<f32>,
    pub offset: Vector2<f32>,
    pub blur: Vector2<f32>,
}

pub trait Effect: Default {
    fn shadow(&self) -> Option<ShadowEffect> {
        None
    }
}
mod no_effect;
mod shadow;

pub use no_effect::NoEffect;
pub use shadow::Shadow;
