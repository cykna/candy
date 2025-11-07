use crate::ui::styling::anim::AnimationCurve;

#[derive(Default)]
pub struct LinearCurve;
impl AnimationCurve for LinearCurve {
    fn calculate(&self, elapsed: f32) -> f32 {
        elapsed
    }
}
