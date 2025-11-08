use std::{
    ops::Deref,
    sync::{
        Arc,
        mpsc::{Receiver, Sender, channel},
    },
    thread,
    time::{Duration, Instant},
};

use crate::{
    ui::{
        animation::{
            AnimationStep, AnyAnimation,
            scheduler::{AnimationScheduler, SchedulerAnimation, SchedulerSender},
        },
        component::Component,
    },
    window::{ComponentEvents, SCHEDULER},
};

///A Component reference that is unsafely, send and sync to be used across threads for scheduling
pub struct ComponentRef(*mut dyn Component);
unsafe impl Send for ComponentRef {}
unsafe impl Sync for ComponentRef {}
impl ComponentRef {
    ///Creates a new component reference
    pub fn new(reference: *mut dyn Component) -> Self {
        Self(reference)
    }
}
impl Deref for ComponentRef {
    type Target = *mut dyn Component;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AwaitingAnimation {
    animation: Arc<dyn AnyAnimation>,
    start_time: Instant,
    target: ComponentRef,
    last_step: Instant,
}

impl AwaitingAnimation {
    #[inline]
    ///Calculates the delta time for this animation. In range to 0..1, 0 is the initial time, 1 is the final time
    pub fn dt(&self) -> f32 {
        self.animation.delta_time(self.start_time.elapsed())
    }
}

pub struct AnimationManager {
    animations: Vec<AwaitingAnimation>,
}

impl AnimationScheduler for AnimationManager {
    fn start_execution(mut self) -> SchedulerSender {
        let mut indices = Vec::new();
        let (tx, rx) = channel::<SchedulerAnimation>();
        let sender = SCHEDULER.retrieve_sender();

        thread::spawn(move || {
            loop {
                while let Ok((anim, config, cref)) = rx.try_recv() {
                    let now = Instant::now() + config.delay;

                    self.animations.push(AwaitingAnimation {
                        animation: anim,
                        start_time: now,
                        target: cref,
                        last_step: Instant::now(),
                    });
                }

                for (idx, anim) in self.animations.iter_mut().enumerate() {
                    if anim.start_time.elapsed() == Duration::ZERO
                        || anim.animation.step_time() > anim.last_step.elapsed()
                    {
                        continue;
                    }
                    if anim.start_time.elapsed() <= anim.animation.duration() {
                        let state = anim.animation.calculate_state(anim.dt());
                        unsafe {
                            state.apply_to(&mut **anim.target);
                        }
                        anim.last_step = Instant::now();
                    } else {
                        indices.push(idx);
                    }
                }
                sender.send(ComponentEvents::CheckUpdates).unwrap();
                indices.reverse();
                for index in indices.drain(..) {
                    self.animations.swap_remove(index);
                }
            }
        });
        tx
    }
    fn insert_animation(&mut self, animation: Arc<dyn AnyAnimation>, target: *mut dyn Component) {
        self.animations.push(AwaitingAnimation {
            animation,
            start_time: Instant::now(),
            target: ComponentRef(target),
            last_step: Instant::now(),
        });
    }
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
        }
    }
}
