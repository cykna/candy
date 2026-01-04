use std::{
    collections::BTreeMap, ops::Deref, sync::Arc, thread, time::{Duration, Instant}
};
use flume::Sender;

use crate::{
    ui::{
        animation::{
            AnimationConfig, AnyAnimation,
            scheduler::{AnimationScheduler, SchedulerAnimation, SchedulerSender},
        },
        component::Component,
    },
    window::{ComponentEvents},
};

///A Component reference that is unsafely, send and sync to be used across threads for scheduling
pub struct ComponentRef<C>(*mut dyn Component<C>);
unsafe impl<C> Send for ComponentRef<C> {}
unsafe impl<C> Sync for ComponentRef<C> {}
impl<C> ComponentRef<C> {
    ///Creates a new component reference
    pub fn new(reference: *mut dyn Component<C>) -> Self {
        Self(reference)
    }
}
impl<C> Deref for ComponentRef<C> {
    type Target = *mut dyn Component<C>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

///An awaiting animation, which is ordered by its steptime
pub struct AwaitingAnimation<C> {
    animation: Arc<dyn AnyAnimation<C>>,
    start_time: Instant,
    target: ComponentRef<C>,
}

impl<C> AwaitingAnimation<C> {
    #[inline]
    ///Calculates the delta time for this animation. In range to 0..1, 0 is the initial time, 1 is the final time
    pub fn dt(&self) -> f32 {
        self.animation.delta_time(self.start_time.elapsed())
    }
}


pub struct AnimationManager<C:'static> {
    animations: BTreeMap<Duration, Vec<AwaitingAnimation<C>>>, //duration is the steptime of the animation
    sender: Sender<ComponentEvents<C>>
}

impl<C:'static> AnimationScheduler<C> for AnimationManager<C> {
    fn command_sender(&mut self) -> flume::Sender<ComponentEvents<C>> {
        self.sender.clone()
    }
    fn start_execution(mut self) -> SchedulerSender<C> {
        let (tx, rx) = flume::unbounded::<SchedulerAnimation<C>>();
        let sender = self.command_sender();
        thread::spawn(move || {
            let mut indices = Vec::new();
            loop {
                if self.animations.is_empty() {
                    if let Ok((animation, config, target)) = rx.recv() {
                        self.insert_animation(animation, *target, config);
                        break;
                    }
                } else if let Ok((animation, config, target)) = rx.try_recv() {
                    self.insert_animation(animation, *target, config);
                    break;
                }
                let mut towait = Duration::ZERO;
                for (duration, anims) in self.animations.iter() {
                    for (idx, animation) in anims.iter().enumerate() {
                        let elapsed = animation.start_time.elapsed();
                        if elapsed == Duration::ZERO {
                            continue;
                        }

                        if elapsed <= animation.animation.duration() {
                            let dt = animation.animation.delta_time(elapsed);
                            let state = animation.animation.calculate_state(dt);
                            state.apply_to(unsafe { &mut **animation.target });
                            let _ = sender.send(ComponentEvents::CheckUpdates);
                        } else {
                            indices.push((*duration, idx));
                        }
                    }
                    std::thread::sleep(*duration - towait);
                    towait = *duration;
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
        animation: Arc<dyn AnyAnimation<C>>,
        target: *mut dyn Component<C>,
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

impl<C> AnimationManager<C> {
    pub fn new(sender:Sender<ComponentEvents<C>>) -> Self {
        Self {
            animations: BTreeMap::new(),
            sender,
        }
    }
}
