pub mod curves;
pub mod manager;
pub mod scheduler;
use std::{sync::Arc, time::Duration};

use crate::ui::{
    animation::{curves::AnimationCurve, manager::ComponentRef, scheduler::SchedulerSender},
    component::Component,
};

#[derive(Default)]
pub struct AnimationConfig {
    pub delay: Duration,
}

///A step to send to the main thread whether the animation finalized or received a new state
pub enum AnimationStep {
    State(Box<dyn AnimationState>),
    Finish,
}

pub trait Animatable<T: AnimationState> {
    fn play_animation(
        &mut self,
        animation: Animation<T>,
        config: AnimationConfig,
        target: SchedulerSender,
    );
}

impl<T: AnimationState + 'static, C> Animatable<T> for C
where
    C: Component + 'static,
{
    ///Starts the provided `animation` on the given `scheduler`
    #[inline]
    fn play_animation(
        &mut self,
        animation: Animation<T>,
        config: AnimationConfig,
        target: SchedulerSender,
    ) {
        let _ = target.send((Arc::new(animation), config, ComponentRef::new(self)));
    }
}

pub trait AnimationState: Send + Sync {
    fn lerp(initial: &Self, end: &Self, t: f32) -> Self
    where
        Self: Sized;
    fn apply_to(&self, comp: &mut dyn Component);
}

pub struct Animation<T: AnimationState> {
    initial: T,
    end: T,
    duration: Duration,
    step_time: Duration,
    curve: Box<dyn AnimationCurve + 'static>,
}

pub trait AnyAnimation: Send + Sync {
    ///Calculates the new state based on the `elapsed` time and the curve this animation uses
    fn calculate_state(&self, elapsed: f32) -> Box<dyn AnimationState>;
    ///Returns the duration of the animation
    fn duration(&self) -> Duration;
    ///Returns the rate the updates are going to be sent
    fn step_time(&self) -> Duration;
    ///Gets the percentage(range from 0..1) the elapsed time have made to reach `duration`. If a duration of 3secs, and `elapsed` == 30ms, then this is 30ms/3sec, thus, 0.01
    fn delta_time(&self, elapsed: Duration) -> f32 {
        elapsed.as_secs_f32() / self.duration().as_secs_f32()
    }
}

impl<T: AnimationState> Animation<T> {
    pub fn new<C: AnimationCurve + std::default::Default + 'static>(
        initial: T,
        end: T,
        duration: Duration,
        step_time: Duration,
    ) -> Self {
        Self {
            end,
            initial,
            duration,
            step_time,
            curve: Box::new(C::default()),
        }
    }
}

impl<T: AnimationState + 'static> AnyAnimation for Animation<T> {
    fn calculate_state(&self, elapsed: f32) -> Box<dyn AnimationState> {
        Box::new(T::lerp(
            &self.initial,
            &self.end,
            self.curve.calculate(elapsed),
        ))
    }
    fn duration(&self) -> Duration {
        self.duration
    }
    fn step_time(&self) -> Duration {
        self.step_time
    }
}
