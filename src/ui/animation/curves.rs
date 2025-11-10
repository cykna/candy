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
#[derive(Default)]
pub struct EaseInOutQuadCurve;

impl AnimationCurve for EaseInOutQuadCurve {
    fn calculate(&self, elapsed: f32) -> f32 {
        if elapsed < 0.5 {
            2.0 * elapsed * elapsed
        } else {
            1.0 - (-2.0 * elapsed + 2.0).powi(2) / 2.0
        }
    }
}

#[derive(Default)]
pub struct BezierCurve;

impl AnimationCurve for BezierCurve {
    fn calculate(&self, t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t)
    }
}

#[derive(Default)]
pub struct ParametricCurve;

impl AnimationCurve for ParametricCurve {
    fn calculate(&self, t:f32) -> f32 {
        let sqr = t * t;
        return sqr / (2.0 * (sqr - t) + 1.0);
    }
}
