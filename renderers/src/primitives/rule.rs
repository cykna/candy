use candy_shared_types::Effect;
#[cfg(feature = "vello")]
use candy_shared_types::Rect;

use candy_shared_types::ShadowEffect;
use candy_shared_types::Style;
use nalgebra::Vector2;
use nalgebra::Vector4;

#[cfg(not(feature = "vello"))]
use skia_safe::{
    Color4f, Point3, Rect,
    image_filters::{self, CropRect},
};

#[cfg(not(feature = "vello"))]
#[derive(Debug, Default)]
pub struct DrawRule {
    pub border_color: Vector4<f32>,
    pub border_radius: Vector2<f32>,
    pub border_width: f32,
    pub inner: Paint,
}

#[derive(Debug)]
pub(crate) enum EffectType {
    Shadow(ShadowEffect),
}

#[cfg(feature = "vello")]
#[derive(Debug, Default)]
pub struct DrawRule {
    pub border_color: Vector4<f32>,
    pub border_radius: Vector2<f32>,
    pub border_width: f32,
    pub color: Vector4<f32>,
    effects: Vec<EffectType>,
}

impl DrawRule {
    pub fn new() -> Self {
        #[cfg(feature = "vello")]
        {
            Self {
                color: Vector4::zeros(),
                border_width: 0.0,
                border_radius: Vector2::zeros(),
                border_color: Vector4::new(0.0, 0.0, 0.0, 0.0),
                effects: Vec::new(),
            }
        }
        #[cfg(not(feature = "vello"))]
        Self {
            inner: Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None),
            border_width: 0.0,
            border_radius: Vector2::zeros(),
            border_color: Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn get_color(&self) -> Vector4<f32> {
        #[cfg(not(feature = "vello"))]
        unsafe {
            std::mem::transmute(self.inner.color4f())
        }
        #[cfg(feature = "vello")]
        self.color
    }

    pub fn set_color(&mut self, color: &Vector4<f32>) {
        #[cfg(not(feature = "vello"))]
        self.inner.set_color4f(vec4f32_to_color(color), None);
        #[cfg(feature = "vello")]
        {
            self.color = color.clone_owned();
        }
    }

    pub fn apply_effect(&mut self, effect: &dyn Effect, mut rect: Rect) {
        #[cfg(not(feature = "vello"))]
        {
            let mut effects = Vec::new();
            if let Some(shadow) = effect.shadow() {
                rect = {
                    let d = shadow.offset; //delta
                    let b = shadow.blur; //blur
                    skia_safe::Rect::new(
                        rect.left + d.x - b.x * 3.0,
                        rect.top + d.y - b.y * 3.0,
                        rect.right + d.x + b.x * 3.0,
                        rect.bottom + d.y + b.y * 3.0,
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
            if effects.is_empty() {
                return;
            }
            let out = image_filters::merge(effects, CropRect::from(&rect));
            self.inner.set_image_filter(out);
        }
        #[cfg(feature = "vello")]
        {
            if let Some(shadow) = effect.shadow() {
                self.effects.push(EffectType::Shadow(shadow));
            }
        }
    }

    pub fn apply_style(&mut self, style: &dyn Style) {
        self.apply_effect(&*style.effect(), Rect::new(0.0, 0.0, 2000.0, 2000.0));

        self.set_color(&style.background_color());
        self.border_color = style.border_color();
        self.border_radius = style.border_radius();
        self.border_width = style.border_width();
    }
}
