use bevy::prelude::*;

#[derive(Component, Hash, Eq, PartialEq, Debug, Default, Clone, Copy)]
pub enum SlimeState {
    Moving,
    #[default]
    Patrolling,
    Dead,
}

pub struct SlimeStatePlugin;
impl Plugin for SlimeStatePlugin {
    fn build(&self, app: &mut App) {
    //    app.add_systems(PostUpdate, switch_slime_state).add_systems(
    //        PostUpdate,
    //        log_slime_state_transitions.after(switch_slime_state),
    //    );
    }
}

//fn switch_slime_state(res_slime_state: Res<State<SlimeState>>) {
//    let slime_state = res_slime_state.get();
//
//    match slime_state {
//        SlimeState::Moving => {}
//        SlimeState::Patrolling => if player_spotted {},
//        SlimeState::Dead => {}
//    }
//}

//fn player_spotted(
//    mut q_slime_transforms: Query<&mut Transform, With<Slime>>,
//    q_player_transform: Query<&Transform, (With<Player>, Without<Slime>)>,
//) {
//    let player_translation = q_player_transform.single().translation;
//
//    for mut transform in &mut q_slime_transforms {
//}
