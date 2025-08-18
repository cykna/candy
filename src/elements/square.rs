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
    pub fn position_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.position
    }

    ///Gets the actual size of this square
    pub fn size_mut(&mut self) -> &mut Vector2<f32> {
        &mut self.size
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

    pub fn with_r(mut self, r: f32) -> Self {
        self.color.x = r;
        self
    }
    pub fn with_g(mut self, g: f32) -> Self {
        self.color.y = g;
        self
    }
    pub fn with_b(mut self, b: f32) -> Self {
        self.color.z = b;
        self
    }
    pub fn with_a(mut self, a: f32) -> Self {
        self.color.w = a;
        self
    }
    pub fn with_color(mut self, color: Vector4<f32>) -> Self {
        self.color = color;
        self
    }
}
