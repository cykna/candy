use std::{
    cmp::Eq,
    collections::BTreeMap,
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
    last_step: Instant,
}

impl Ord for AwaitingAnimation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.animation.step_time().cmp(&other.animation.step_time())
    }
}
impl PartialOrd for AwaitingAnimation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.animation.step_time().cmp(&other.animation.step_time()))
    }
}
impl PartialEq for AwaitingAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.animation.step_time() == other.animation.step_time()
    }
}
impl Eq for AwaitingAnimation {}

impl AwaitingAnimation {
    #[inline]
    ///Calculates the delta time for this animation. In range to 0..1, 0 is the initial time, 1 is the final time
    pub fn dt(&self) -> f32 {
        self.animation.delta_time(self.start_time.elapsed())
    }
}

pub struct AnimationManager {
    anim_rx: Receiver<SchedulerAnimation>,
    tx: Sender<SchedulerAnimation>,
    animations: BTreeMap<Duration, Vec<AwaitingAnimation>>, //duration is the steptime of the animation
}

impl AnimationScheduler for AnimationManager {
    fn start_execution(mut self) -> SchedulerSender {
        let mut indices = Vec::new();

        let sender = SCHEDULER.retrieve_sender();
        let outx = self.tx.clone();
        let mut idx = 0;
        thread::spawn(move || {
            loop {
                if self.animations.is_empty() {
                    while let Ok((animation, config, target)) = self.anim_rx.recv() {
                        self.insert_animation(animation, *target, config);
                        break;
                    }
                }

                while let Ok((animation, config, target)) = self.anim_rx.try_recv() {
                    self.insert_animation(animation, *target, config);
                    break;
                }
                let mut towait = Duration::ZERO;
                for (duration, anims) in self.animations.iter() {
                    println!("Oh quantas animações com {duration:?}: {}", anims.len());
                    for (idx, animation) in anims.iter().enumerate() {
                        if animation.start_time.elapsed() == Duration::ZERO {
                            continue;
                        }
                        //1 Anim && 2 Anim, 16ms -> Exec, wait, 16ms
                        //2 Anim, 4ms, -> Exec, wait  4ms
                        //1 Anim

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
                    println!("waiting {towait:?} {idx}");
                    idx += 1;
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
        outx
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
            last_step: Instant::now(),
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
        let (tx, anim_rx) = channel::<SchedulerAnimation>();
        Self {
            tx,
            anim_rx,
            animations: BTreeMap::new(),
        }
    }
}
