use std::{
    collections::BTreeMap,
    ops::Deref,
    sync::{Arc, mpsc::channel},
    thread,
    time::{Duration, Instant},
};

use crate::{
    ui::{
        animation::{
            AnimationConfig, AnyAnimation,
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

///An awaiting animation, which is ordered by its steptime
pub struct AwaitingAnimation {
    animation: Arc<dyn AnyAnimation>,
    start_time: Instant,
    target: ComponentRef,
}

impl AwaitingAnimation {
    #[inline]
    ///Calculates the delta time for this animation. In range to 0..1, 0 is the initial time, 1 is the final time
    pub fn dt(&self) -> f32 {
        self.animation.delta_time(self.start_time.elapsed())
    }
}

pub struct AnimationManager {
    animations: BTreeMap<Duration, Vec<AwaitingAnimation>>, //duration is the steptime of the animation
}

impl AnimationScheduler for AnimationManager {
    fn start_execution(mut self) -> SchedulerSender {
        let (tx, rx) = channel::<SchedulerAnimation>();

        thread::spawn(move || {
            let mut indices = Vec::new();
            let sender = SCHEDULER.retrieve_sender();
            loop {
                if self.animations.is_empty() {
                    while let Ok((animation, config, target)) = rx.recv() {
                        self.insert_animation(animation, *target, config);
                        break;
                    }
                } else {
                    while let Ok((animation, config, target)) = rx.try_recv() {
                        self.insert_animation(animation, *target, config);
                        break;
                    }
                }
                let mut towait = Duration::ZERO;
                for (duration, anims) in self.animations.iter() {
                    for (idx, animation) in anims.iter().enumerate() {
                        if animation.start_time.elapsed() == Duration::ZERO {
                            continue;
                        }

                        let elapsed = animation.start_time.elapsed();
                        if elapsed <= animation.animation.duration() {
                            let state = animation.animation.calculate_state(elapsed.as_secs_f32());
                            state.apply_to(unsafe { &mut **animation.target });
                            let _ = sender.send(ComponentEvents::CheckUpdates);
                        } else {
                            indices.push((*duration, idx));
                        }
                    }
                    towait += *duration - towait;
                    std::thread::sleep(towait);
                }

                for (ref dur, idx) in indices.drain(..).rev() {
                    if let Some(vec) = self.animations.get_mut(dur) {
                        vec.swap_remove(idx);
                        if vec.is_empty() {
                            self.animations.remove(dur);
                        }
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
        let anim = AwaitingAnimation {
            animation: animation.clone(),
            start_time: Instant::now() + config.delay,
            target: ComponentRef(target),
        };
        if let Some(vec) = self.animations.get_mut(&animation.step_time()) {
            vec.push(anim);
        } else {
            self.animations.insert(animation.step_time(), vec![anim]);
        };
    }
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            animations: BTreeMap::new(),
        }
    }
}
