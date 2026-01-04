use std::sync::Arc;

use flume::Sender;

use crate::{ui::{
    animation::{AnimationConfig, AnyAnimation, manager::ComponentRef},
    component::Component,
}, window::ComponentEvents};

pub type SchedulerAnimation<C> = (Arc<dyn AnyAnimation<C>>, AnimationConfig, ComponentRef<C>);
pub type SchedulerSender<C> = Sender<SchedulerAnimation<C>>;

///A scheduler that will run on a separated thread to manage concurrently the threads to be runned
pub trait AnimationScheduler<C>: 'static {
    fn command_sender(&mut self) -> Sender<ComponentEvents<C>>;
    ///The starting execution of the scheduler. This is executed on the current thread and does not creates a new one, instead the programmer must define it when this should be executed
    ///and if on the main or, a separeted one
    fn start_execution(self) -> SchedulerSender<C>;
    fn insert_animation(
        &mut self,
        animation: Arc<dyn AnyAnimation<C>>,
        target: *mut dyn Component<C>,
        config: AnimationConfig,
    );
}
