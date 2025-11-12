use nalgebra::Vector2;

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
///A rectangle containing position and size.
pub struct Rect {
    ///The position of the rect on the X axis
    pub x: f32,
    ///The position of the rect on the Y axis
    pub y: f32,
    ///The width of the rect
    pub width: f32,
    ///The height of the rect
    pub height: f32,
}

impl Rect {
    #[inline]
    ///The X position of the right corner
    pub fn right(&self) -> f32 {
        self.x + self.width
    }
    #[inline]
    ///The Y position of the bottom corner
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    #[inline]
    ///Checks whether the given `pos` is inside this `rect`
    pub fn contains(&self, pos: Vector2<f32>) -> bool {
        pos.x >= self.x && pos.y >= self.y && pos.x <= self.right() && pos.y <= self.bottom()
    }
    ///Retrieves the center position of this rect
    pub fn center(&self) -> Vector2<f32> {
        Vector2::new(self.x + self.width * 0.5, self.y + self.height * 0.5)
    }
}
