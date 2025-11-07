use crate::{
    renderer::CandyRenderer,
    ui::{component::Component, styling::anim::AnimationState},
};

pub struct AnimationManager<'a, R: CandyRenderer> {
    awaiting: Vec<(&'a dyn Component, Box<dyn AnimationState>)>,
}
