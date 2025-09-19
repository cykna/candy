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

///Gets the size of a char assuming it's first byte is the provided `byte`
pub fn char_size_init(mut byte: u8) -> u8 {
    if byte < 0x80 {
        1
    } else {
        byte &= 0b1111_0000;
        let mut char_size = 0;
        while byte != 0 {
            char_size += 1;
            byte <<= 1;
        }
        char_size - 1
    }
}
///On starting by `start_point` iterates to the left searching for a byte that can determine the size required to represent that byte.
///Returns the amount of bytes needed to represent a char that contains the byte at `starting_point`. 0 if it's not utf8 valid
///Example
///```rust
///let v = vec![b'a', 0xc3, 0xa0, 0xc3, 0xa7]; //'aàç'
///char_size_backwards(&v, v.len()-1); //2
///char_size_backwards(&v, v.len()-3); //2
///char_size_backwards(&v, 1); // 0; Invalid Byte
///char_size_backwards(&v, 0); //1
///```
pub fn char_size_backwards(vec: &[u8], start_point: usize) -> usize {
    if vec[start_point] < 0x80 {
        1
    } else {
        let mut byte_count = 0;
        while byte_count < 4 {
            let mut c = vec[start_point - byte_count] & 0b1111_0000;
            if c > 0b1000_0000 {
                let mut n = 0;
                while c != 0 {
                    n += 1;
                    c <<= 1;
                }
                return n - 1;
            }
            byte_count += 1;
        }
        0
    }
}
