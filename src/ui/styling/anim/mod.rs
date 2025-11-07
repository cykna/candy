pub mod curves;
pub mod manager;
use crate::renderer::CandyRenderer;
use std::{
    marker::PhantomData,
    sync::{
        Arc,
        mpsc::{Receiver, channel},
    },
    thread,
    time::{Duration, Instant},
};

use crate::ui::component::Component;

pub trait Animatable<T: AnimationState<R>, R: CandyRenderer> {
    fn play_animation(&mut self, animation: Animation<T, R>) -> Receiver<T>;
}

impl<T: AnimationState<R> + 'static, R, C> Animatable<T, R> for C
where
    R: CandyRenderer,
    C: Component<R>,
{
    fn play_animation(&mut self, animation: Animation<T, R>) -> Receiver<T> {
        let (tx, rx) = channel();
        let arc = Arc::new(animation);
        thread::spawn(move || {
            let anim = arc.clone();
            let now = Instant::now();
            while now.elapsed() < anim.duration {
                let state = anim.calculate_state(now.elapsed().as_secs_f32());
                if let Err(_) = tx.send(state) {
                    break;
                };
                std::thread::sleep(anim.step_time);
            }
        });
        rx
    }
}
///Simply a curve that calculates the value used to lerp between 2 states based on the `elapsed` time
pub trait AnimationCurve: Send + Sync {
    fn calculate(&self, elapsed: f32) -> f32;
}

pub trait AnimationState: Send + Sync {
    fn lerp(initial: &Self, end: &Self, t: f32) -> Self;
    fn apply_to(self, comp: &mut dyn Component);
}

pub struct Animation<T: AnimationState, R: CandyRenderer> {
    phantom: PhantomData<R>,
    initial: T,
    end: T,
    duration: Duration,
    step_time: Duration,
    curve: Box<dyn AnimationCurve + 'static>,
}

impl<T: AnimationState<R>, R: CandyRenderer> Animation<T, R> {
    pub fn new<C: AnimationCurve + std::default::Default + 'static>(
        initial: T,
        end: T,
        duration: Duration,
        step_time: Duration,
    ) -> Self {
        Self {
            phantom: PhantomData,
            end,
            initial,
            duration,
            step_time,
            curve: Box::new(C::default()),
        }
    }
    ///Calculates the new state based on the `elapsed` time and the curve this animation uses
    pub fn calculate_state(&self, elapsed: f32) -> T {
        T::lerp(&self.initial, &self.end, elapsed)
    }
}
