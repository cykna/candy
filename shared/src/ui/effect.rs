use nalgebra::{Vector2, Vector4};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
///The specification of how the shadow effect should be applies
pub struct ShadowEffect {
    ///The color of the shadow
    pub color: Vector4<f32>,
    ///The offset of the shadow when applied
    pub offset: Vector2<f32>,
    ///The blur power on X and Y axis
    pub blur: Vector2<f32>,
}

///An effect to be applied to a component when rendering
pub trait Effect {
    ///Retrieves whether this effect has or not a ShadowEffect
    fn shadow(&self) -> Option<ShadowEffect> {
        None
    }
}

#[derive(Debug, Clone, Default)]
///A shadow effect to be applied on the component
pub struct Shadow {
    color: Vector4<f32>,
    offset: Vector2<f32>,
    blur: Vector2<f32>,
}

///A Effect that doesn't apply any effect actually.
pub struct NoEffect;
impl Effect for NoEffect {}

impl Shadow {
    ///Creates a new Shadow with the provided `color`
    pub fn colored(color: Vector4<f32>) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    ///Sets the color of this shadow to be the given `color`
    pub fn with_color(mut self, color: Vector4<f32>) -> Self {
        self.color = color;
        self
    }
    #[inline]
    ///Sets the blur of this shadow to be ghe given `blur`
    pub fn with_blur(mut self, blur: Vector2<f32>) -> Self {
        self.blur = blur;
        self
    }

    #[inline]
    ///Sets the blur of this shadow to be ghe given `offset`
    pub fn with_offset(mut self, offset: Vector2<f32>) -> Self {
        self.offset = offset;
        self
    }
}

impl Effect for Shadow {
    fn shadow(&self) -> Option<super::ShadowEffect> {
        Some(ShadowEffect {
            color: self.color,
            blur: self.blur,
            offset: self.offset,
        })
    }
}
