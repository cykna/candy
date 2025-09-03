pub mod rect;
use nalgebra::{Vector2, Vector4};
use rect::Rect;
use skia_safe::Color4f;

///Casts the given vector into a Color4f.
pub fn vec4f32_to_color(color: &Vector4<f32>) -> &Color4f {
    unsafe { std::mem::transmute::<&Vector4<f32>, &Color4f>(color) }
}

///Casts the given vector into a Color4f.
pub fn vec4f32_to_color_value(color: Vector4<f32>) -> Color4f {
    unsafe { std::mem::transmute::<Vector4<f32>, Color4f>(color) }
}

pub fn vec4f32_to_rect(color: &Vector4<f32>) -> &Rect {
    unsafe { std::mem::transmute::<&Vector4<f32>, &Rect>(color) }
}

///Checks weather `position` is inside of `rect`. This function interprets zw of `rect` as it's width and height
pub fn in_bounds_of(rect: Vector4<f32>, position: Vector2<f32>) -> bool {
    (position.x >= rect.x && position.x <= rect.x + rect.z)
        && (position.y >= rect.y && position.y <= rect.y + rect.w)
}

#[inline]
///Retrieves the position where the `child` square will be centered on `parent` square
pub fn center(child: &Rect, parent: &Rect) -> Vector2<f32> {
    let center = parent.center();
    Vector2::new(center.x - child.width * 0.5, center.y + child.height * 0.5)
}
