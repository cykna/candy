use nalgebra::Vector2;

#[repr(C)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    #[inline]
    pub fn right(&self) -> f32 {
        self.x + self.width
    }
    #[inline]
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    #[inline]
    ///Checks weather the given `pos` is
    pub fn contains(&self, pos: Vector2<f32>) -> bool {
        pos.x >= self.x && pos.y >= self.y && pos.x <= self.right() && pos.y <= self.bottom()
    }
}
