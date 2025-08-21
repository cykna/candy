use nalgebra::{Vector2, Vector4};
pub struct ShadowEffect {
    pub offset: Vector2<f32>,
    pub color: Vector4<f32>,
    pub blur: Vector2<f32>,
}

pub trait Effect {
    fn shadow(&self) -> Option<ShadowEffect> {
        None
    }
}
mod no_effect;

pub use no_effect::NoEffect;
