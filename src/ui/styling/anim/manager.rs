use std::sync::mpsc::Receiver;

use crate::ui::{component::Component, styling::anim::AnimationState};

#[derive(Debug)]
pub struct AnimationManager {
    awaiting: Vec<(
        *mut dyn Component,
        Receiver<Box<dyn AnimationState + 'static>>,
    )>,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            awaiting: Vec::new(),
        }
    }
    pub fn insert_awaiter(
        &mut self,
        component: *mut dyn Component,
        recv: Receiver<Box<dyn AnimationState + 'static>>,
    ) {
        self.awaiting.push((component, recv));
    }

    pub fn update(&mut self) -> bool {
        let mut flag = false;
        for (comp, awaiter) in self.awaiting.iter() {
            let Ok(val) = awaiter.try_recv() else {
                continue;
            };
            flag = true;
            unsafe {
                val.apply_to(&mut **comp);
            }
        }

        flag
    }
}
