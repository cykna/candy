use nalgebra::{Vector2, Vector4};

///A handler that contains information about how a square should be drawn.
#[derive(Debug, Default)]
pub struct CandySquare {
    border_color: Vector4<f32>,
    position: Vector2<f32>,
    size: Vector2<f32>,
    border_radius: Vector2<f32>,
    border_width: f32,
    pub(crate) dirty: bool,
}

impl CandySquare {
    pub fn new(
        position: Vector2<f32>,
        size: Vector2<f32>,
        border: Option<Vector4<f32>>,
        radius: Option<Vector2<f32>>,
    ) -> Self {
        Self {
            position,
            size,
            border_color: border.unwrap_or(Vector4::zeros()),
            border_radius: radius.unwrap_or(Vector2::zeros()),
            border_width: 0.0,
            dirty: true,
        }
    }

    ///Sets the border width of this square
    pub fn with_border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    ///Retrieves the border width of this square
    #[inline]
    pub fn border_width(&self) -> f32 {
        self.border_width
    }

    ///Gets the position of this square
    ///Obs: As this gets mutable, this code assumes the data will be changed, so, this is marked as dirty
    pub fn position_mut(&mut self) -> &mut Vector2<f32> {
        self.dirty = true;
        &mut self.position
    }

    ///Gets the actual size of this square
    ///Obs: As this gets mutable, this code assumes the data will be changed, so, this is marked as dirty    ///
    pub fn size_mut(&mut self) -> &mut Vector2<f32> {
        self.dirty = true;
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

    ///Gets the border color of this square
    pub fn border_color(&self) -> &Vector4<f32> {
        &self.border_color
    }

    ///Gets the border radius of this square
    pub fn border_radius(&self) -> &Vector2<f32> {
        &self.border_radius
    }
}
