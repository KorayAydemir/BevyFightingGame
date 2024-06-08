use bevy::prelude::*;

use super::input::PlayerInput;
use super::state::PlayerState;
use super::Player;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 50.;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
    }
}

fn player_movement(
    mut q_player_transform: Query<&mut Transform, With<Player>>,
    mut q_player: Query<&mut Player>,
    player_input: Res<PlayerInput>,
) {
    let mut transform = q_player_transform.single_mut();
    let mut player = q_player.single_mut();

    let translation = &mut transform.translation;
    translation.x += player_input.move_direction.x * TIME_STEP * BASE_SPEED;
    translation.y += player_input.move_direction.y * TIME_STEP * BASE_SPEED;

    player.state = PlayerState::Moving;
}
