use bevy::prelude::*;

use crate::{player::Player, GameState};

use super::Slime;

const BASE_SPEED: f32 = 50.;

pub struct SlimeMovementPlugin;
impl Plugin for SlimeMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement(
    mut q_slime_transforms: Query<&mut Transform, With<Slime>>,
    q_player_transform: Query<&Transform, (With<Player>, Without<Slime>)>,
    time: Res<Time>
) {
    let player_translation = q_player_transform.single().translation;

    for mut transform in &mut q_slime_transforms {
        let translation = &mut transform.translation;

        let x_direction_multiplier = if translation.x > player_translation.x {
            -1.
        } else {
            1.
        };

        let y_direction_multiplier = if translation.y > player_translation.y {
            -1.
        } else {
            1.
        };

        translation.x += time.delta_seconds() * BASE_SPEED * x_direction_multiplier;
        translation.y += time.delta_seconds() * BASE_SPEED * y_direction_multiplier;
    }
}
