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
    player_state: Res<State<PlayerState>>,
) {
    let mut transform = q_player_transform.single_mut();
    let player_state = player_state.get();

    let PlayerState::Moving(direction) = player_state else {
        return;
    };

    let x_direction_multiplier = if let Some(horizontal) = direction.horizontal {
        match horizontal {
            Horizontal::Right => 1.,
            Horizontal::Left => -1.,
        }
    } else {
        0.
    };

    let y_direction_multiplier = if let Some(vertical) = direction.vertical {
        match vertical {
            Vertical::Up => 1.,
            Vertical::Down => -1.,
        }
    } else {
        0.
    };

    let translation = &mut transform.translation;
    translation.x += TIME_STEP * BASE_SPEED * x_direction_multiplier;
    translation.y += TIME_STEP * BASE_SPEED * y_direction_multiplier;
}
