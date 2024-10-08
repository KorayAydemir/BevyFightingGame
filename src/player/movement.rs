use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use super::state::PlayerState;
use super::{Player, PlayerSet};
use crate::common::movement::{Horizontal, Vertical};

const BASE_SPEED: f32 = 200.;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement.in_set(PlayerSet))
            .add_systems(Update, set_velocity_to_zero);
    }
}

fn set_velocity_to_zero(mut q_player: Query<&mut Velocity, With<Player>>) {
    let mut player_velocity = q_player.single_mut();
    player_velocity.linvel = Vec2::ZERO;
}

fn player_movement(
    mut q_player_transform: Query<&mut Transform, With<Player>>,
    player_state: Res<State<PlayerState>>,
    time: Res<Time>,
) {
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

    let translation = &mut q_player_transform.single_mut().translation;
    translation.y += time.delta_seconds() * BASE_SPEED * y_direction_multiplier;
    translation.x += time.delta_seconds() * BASE_SPEED * x_direction_multiplier;
}
