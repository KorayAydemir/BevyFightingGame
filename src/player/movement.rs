use bevy::prelude::*;

use super::input::PlayerInput;
use super::state::PlayerState;
use super::Player;

const TIME_STEP: f32 = 1. / 60.;
// 16ms
const BASE_SPEED: f32 = 65.;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
    }
}

fn player_movement(
    mut q_player_transform: Query<&mut Transform, With<Player>>,
    mut q_player: Query<&mut Player>,
    mut q_player_sprite: Query<&mut Sprite, With<Player>>,
    player_input: Res<PlayerInput>,
) {
    let mut transform = q_player_transform.single_mut();
    let mut player = q_player.single_mut();

     if player.state != PlayerState::Moving {
         return;
     }

    let translation = &mut transform.translation;
    translation.x += player_input.move_direction.x * TIME_STEP * BASE_SPEED;
    translation.y += player_input.move_direction.y * TIME_STEP * BASE_SPEED;

    let mut player_sprite = q_player_sprite.get_single_mut().unwrap();
    player_sprite.flip_x = player_input.move_direction.x < 0.;

    player.state = PlayerState::Moving;
}
