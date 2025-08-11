use nalgebra::{Vector2, Vector4};
use taffy::Layout;

///A handler that contains information about how a square should be drawn.
#[derive(Debug, Default)]
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
    ///Gets the position of this square
    pub fn position(&self) -> &Vector2<f32> {
        &self.position
    }
    ///Gets the actual size of this square
    pub fn size(&self) -> &Vector2<f32> {
        &self.size
    }
    ///Gets the color of this square
    pub fn background_color(&self) -> &Vector4<f32> {
        &self.color
    }

    ///Gets the border color of this square
    pub fn border_color(&self) -> &Vector4<f32> {
        &self.border_color
    }

    ///Gets the border radius of this square
    pub fn border_radius(&self) -> &Vector2<f32> {
        &self.border_radius
    }

    pub fn resize(&mut self, layout: &Layout) {
        self.position.x = layout.location.x;
        self.position.y = layout.location.y;
        self.size.x = layout.size.width;
        self.size.y = layout.size.height
    }
}
