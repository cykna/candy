use nalgebra::Vector4;
use skia_safe::Color4f;

///Casts the given vector into a Color4f.
pub fn vec4f32_to_color(color: &Vector4<f32>) -> &Color4f {
    unsafe { std::mem::transmute::<&Vector4<f32>, &Color4f>(color) }
}
