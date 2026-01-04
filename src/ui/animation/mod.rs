pub mod curves;
pub mod manager;
pub mod scheduler;
use std::{marker::PhantomData, sync::Arc, time::Duration};

use crate::ui::{
    animation::{curves::AnimationCurve, manager::ComponentRef, scheduler::SchedulerSender},
    component::Component,
};

#[derive(Default)]
pub struct AnimationConfig {
    pub delay: Duration,
}

///A step to send to the main thread whether the animation finalized or received a new state
pub enum AnimationStep<C> {
    State(Box<dyn AnimationState<C>>),
    Finish,
}

pub trait Animatable<T: AnimationState<C>, C:Send+Sync> {
    fn play_animation(
        &mut self,
        animation: Animation<T,C>,
        config: AnimationConfig,
        target: SchedulerSender<C>,
    );
}

impl<T: AnimationState<Command> + 'static, C, Command:Send+Sync> Animatable<T, Command> for C
where
    Command: 'static,
    C: Component<Command> + 'static,
{
    ///Starts the provided `animation` on the given `scheduler`
    #[inline]
    fn play_animation(
        &mut self,
        animation: Animation<T,Command>,
        config: AnimationConfig,
        target: SchedulerSender<Command>,
    ) {
        let _ = target.send((Arc::new(animation), config, ComponentRef::new(self)));
    }
}

pub trait AnimationState<C>: Send + Sync {
    ///Executed to get an intermediate state to when executing an animation.
    ///`initial` is the initial value that the animation started this, initialized from. `end` is the value that the animation if going towards. `cdt`, which is 'curve delta time' is the
    ///delta time that passed after applying the curve of the animation initialized it, and `dt` is the delta time since the start
    fn lerp(initial: &Self, end: &Self, cdt: f32, dt: f32) -> Self
    where
        Self: Sized;
    fn apply_to(&self, comp: &mut dyn Component<C>);
}

pub struct Animation<T: AnimationState<C>, C:'static+Send+Sync> {
    phantom: PhantomData<C>,
    initial: T,
    end: T,
    duration: Duration,
    step_time: Duration,
    curve: Box<dyn AnimationCurve + 'static>,
}

pub trait AnyAnimation<C>: Send + Sync {
    ///Calculates the new state based on the `elapsed` time, which is the delta time since the start of the animation
    fn calculate_state(&self, elapsed: f32) -> Box<dyn AnimationState<C>>;
    ///Returns the duration of the animation
    fn duration(&self) -> Duration;
    ///Returns the rate the updates are going to be sent
    fn step_time(&self) -> Duration;
    ///Gets the percentage(range from 0..1) the elapsed time have made to reach `duration`. If a duration of 3secs, and `elapsed` == 30ms, then this is 30ms/3sec, thus, 0.01
    fn delta_time(&self, elapsed: Duration) -> f32 {
        elapsed.as_secs_f32() / self.duration().as_secs_f32()
    }
}

impl<T: AnimationState<Command>, Command:Send+Sync> Animation<T, Command> {
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
}

impl<C:'static + Send + Sync,T: AnimationState<C> + 'static> AnyAnimation<C> for Animation<T,C> {
    fn calculate_state(&self, elapsed: f32) -> Box<dyn AnimationState<C>> {
        Box::new(T::lerp(
            &self.initial,
            &self.end,
            self.curve.calculate(elapsed),
            elapsed,
        ))
    }
    fn duration(&self) -> Duration {
        self.duration
    }
    fn step_time(&self) -> Duration {
        self.step_time
    }
}
