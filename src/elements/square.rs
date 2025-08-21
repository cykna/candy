use nalgebra::{Vector2, Vector4};

///A handler that contains information about how a square should be drawn.
#[derive(Debug, Default)]
pub struct CandySquare {
    color: Vector4<f32>,
    border_color: Vector4<f32>,
    position: Vector2<f32>,
    size: Vector2<f32>,
    border_radius: Vector2<f32>,
    pub(crate) dirty: bool,
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
            dirty: true,
        }
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

    ///Modifies the red of the color of this square to be `r`
    pub fn with_r(mut self, r: f32) -> Self {
        self.dirty = true;
        self.color.x = r;
        self
    }

    ///Modifies the green of the color of this square to be `g`
    pub fn with_g(mut self, g: f32) -> Self {
        self.dirty = true;
        self.color.y = g;
        self
    }
    ///Modifies the blue of the color of this square to be `b`
    pub fn with_b(mut self, b: f32) -> Self {
        self.dirty = true;
        self.color.z = b;
        self
    }
    ///Modifies the alpha of the color of this square to be `a`
    pub fn with_a(mut self, a: f32) -> Self {
        self.dirty = true;
        self.color.w = a;
        self
    }
    ///Modifies the color of this square to be the given `color`
    #[inline]
    pub fn with_color(mut self, color: Vector4<f32>) -> Self {
        self.dirty = true;
        self.color = color;
        self
    }
}
