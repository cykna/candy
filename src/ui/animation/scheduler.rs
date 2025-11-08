use std::sync::{Arc, mpsc::Sender};

use crate::ui::{
    animation::{AnimationConfig, AnyAnimation, manager::ComponentRef},
    component::Component,
};

pub type SchedulerAnimation = (Arc<dyn AnyAnimation>, AnimationConfig, ComponentRef);
pub type SchedulerSender = Sender<SchedulerAnimation>;

///A scheduler that will run on a separated thread to manage concurrently the threads to be runned
pub trait AnimationScheduler: 'static {
    ///The starting execution of the scheduler. This is executed on the current thread and does not creates a new one, instead the programmer must define it when this should be executed
    ///and if on the main or, a separeted one
    fn start_execution(self) -> SchedulerSender;
    fn insert_animation(&mut self, animation: Arc<dyn AnyAnimation>, target: *mut dyn Component);
}
