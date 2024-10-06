use crate::{common::movement::Direction, impl_can_move};
use bevy::prelude::*;

#[derive(States, Hash, Eq, PartialEq, Debug, Default, Clone)]
pub enum KoalaState {
    #[default]
    Idling,
    Moving(Direction),
}
impl_can_move!(KoalaState);

pub struct KoalaStatePlugin;
impl Plugin for KoalaStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<KoalaState>()
            .add_systems(PostUpdate, switch_koala_state);
    }
}

fn switch_koala_state(koala_state: Res<State<KoalaState>>) {
    let koala_state = koala_state.get();
    match koala_state {
        KoalaState::Idling => {}
        KoalaState::Moving(_) => {}
    }
}
