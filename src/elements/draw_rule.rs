use nalgebra::Vector4;
use skia_safe::Blender;
use skia_safe::Color4f;
use skia_safe::image_filters;
use skia_safe::image_filters::CropRect;

use skia_safe::ColorChannel;
use skia_safe::ColorFilter;
use skia_safe::Paint;
use skia_safe::Point;
use skia_safe::Rect;
use skia_safe::SamplingOptions;

use crate::helpers::vec4f32_to_color;
use crate::helpers::vec4f32_to_color_value;
use crate::ui::styling::fx::Effect;

#[derive(Debug, Default)]
pub struct DrawRule {
    pub(crate) inner: Paint,
}

impl DrawRule {
    pub fn new() -> Self {
        Self {
            inner: Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None),
        }
    }

    pub fn get_color(&self) -> Vector4<f32> {
        unsafe { std::mem::transmute(self.inner.color4f()) }
    }

    pub fn set_color(&mut self, color: &Vector4<f32>) {
        self.inner.set_color4f(vec4f32_to_color(color), None);
    }

    pub fn apply_effect(&mut self, effect: impl Effect, mut rect: Rect) {
        let mut effects = Vec::new();
        if let Some(shadow) = effect.shadow() {
            rect = {
                let d = shadow.offset; //delta
                let b = shadow.blur; //blur
                skia_safe::Rect::new(
                    rect.left + d.x - b.x * 2.5,
                    rect.top + d.y - b.y * 2.5,
                    rect.right + d.x + b.x * 2.5,
                    rect.bottom + d.y + b.y * 2.5,
                )
            };
            effects.push(image_filters::drop_shadow(
                Point::new(shadow.offset.x, shadow.offset.y),
                (shadow.blur.x, shadow.blur.y),
                vec4f32_to_color_value(shadow.color),
                None,
                None,
                CropRect::from(rect),
            ))
        };

        let out = image_filters::merge(effects, CropRect::from(&rect));
        self.inner.set_image_filter(out);
    }
}
