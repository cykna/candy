use nalgebra::{Vector2, Vector4};

pub struct CandySquare {
    color: Vector4<f32>,
    border_color: Vector4<f32>,
    position: Vector2<f32>,
    size: Vector2<f32>,
    border_radius: Vector2<f32>,
}

impl CandySquare {
    pub fn new(
        position: Vector2<f32>,
        size: Vector2<f32>,
        color: Vector4<f32>,
        border: Option<Vector4<f32>>,
        radius: Option<Vector2<f32>>,
    ) -> Self {
        Self {
            position,
            size,
            color,
            border_color: border.unwrap_or(Vector4::zeros()),
            border_radius: radius.unwrap_or(Vector2::zeros()),
        }
    }
    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }
    pub fn size(&self) -> &Vector2<f32> {
        &self.size
    }
    pub fn background_color(&self) -> &Vector4<f32> {
        &self.color
    }
    pub fn border_color(&self) -> &Vector4<f32> {
        &self.border_color
    }
    pub fn border_radius(&self) -> &Vector2<f32> {
        &self.border_radius
    }
}
