use bevy::prelude::*;

use super::input::{Horizontal, Vertical};
use super::state::PlayerState;
use super::Player;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 65.;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
    }
}

fn player_movement(
    mut q_player_transform: Query<&mut Transform, With<Player>>,
    q_player: Query<&Player>,
    mut q_player_sprite: Query<&mut Sprite, With<Player>>,
) {
    let player = q_player.single();
    let mut transform = q_player_transform.single_mut();

    let PlayerState::Moving(direction) = player.state else {
        return;
    };

    let x_direction_multiplier = if let Some(horizontal) = direction.horizontal {
        match horizontal {
            Horizontal::Right => 1.,
            Horizontal::Left => { 
                let mut player_sprite = q_player_sprite.get_single_mut().unwrap();
                player_sprite.flip_x = true;
                -1. 
            }
        }
    } else {
        0.
    };

    let y_direction_multiplier = if let Some(vertical) = direction.vertical {
        match vertical {
            Vertical::Up => 1.,
            Vertical::Down => -1.
        }
    } else {
        0.
    };

    let translation = &mut transform.translation;
    translation.x += TIME_STEP * BASE_SPEED * x_direction_multiplier;
    translation.y += TIME_STEP * BASE_SPEED * y_direction_multiplier;

}
