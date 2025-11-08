///Simply a curve that calculates the value used to lerp between 2 states based on the `elapsed` time
pub trait AnimationCurve: Send + Sync {
    fn calculate(&self, elapsed: f32) -> f32;
}

#[derive(Default)]
pub struct LinearCurve;

impl AnimationCurve for LinearCurve {
    fn calculate(&self, elapsed: f32) -> f32 {
        elapsed
    }
}
