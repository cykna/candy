use crate::ui::{component::Component, styling::anim::AnimationState};

pub struct AnimationManager<'a> {
    awaiting: Vec<(&'a dyn Component, Box<dyn AnimationState>)>,
}
