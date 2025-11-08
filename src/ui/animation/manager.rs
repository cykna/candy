use std::{
    ops::Deref,
    sync::{
        Arc,
        mpsc::{Receiver, RecvTimeoutError, Sender, channel},
    },
    thread,
    time::{Duration, Instant},
};

use crate::{
    ui::{
        animation::{
            AnimationConfig, AnimationStep, AnyAnimation,
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
        let mut remaining = Duration::from_secs(1);
        thread::spawn(move || {
            'outer: loop {
                if self.animations.is_empty() {
                    while let Ok((animation, config, target)) = rx.recv() {
                        self.insert_animation(animation, *target, config);
                        break;
                    }
                }
                for (idx, anim) in self.animations.iter_mut().enumerate() {
                    if anim.start_time.elapsed() == Duration::ZERO {
                        continue;
                    }
                    let elapsed = anim.last_step.elapsed();
                    if anim.animation.step_time() > elapsed
                    //the step_time == Duration::from_ms(7), last_step == Duration::from_ms(3), so theres no need to update yet
                    {
                        let dt = anim.animation.step_time() - elapsed;
                        if remaining > dt {
                            remaining = dt;
                        }
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
                {
                    sender.send(ComponentEvents::CheckUpdates).unwrap();

                    for index in indices.drain(..).rev() {
                        self.animations.swap_remove(index);
                    }
                }
                loop {
                    match rx.recv_timeout(remaining) {
                        Ok((anim, config, target)) => {
                            self.insert_animation(anim, *target, config);
                            break;
                        }
                        Err(RecvTimeoutError::Timeout) => break,
                        Err(RecvTimeoutError::Disconnected) => break 'outer,
                    }
                }
            }
        });
        tx
    }
    fn insert_animation(
        &mut self,
        animation: Arc<dyn AnyAnimation>,
        target: *mut dyn Component,
        config: AnimationConfig,
    ) {
        self.animations.push(AwaitingAnimation {
            animation,
            start_time: Instant::now() + config.delay,
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
