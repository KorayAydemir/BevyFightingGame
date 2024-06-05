use bevy::prelude::*;

use super::Player;

use super::spawn::Velocity;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 50.;

pub struct PlayerMovementPlugin;
impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
    }
}

fn player_movement(mut q_player: Query<(&Velocity, &mut Transform), With<Player>>) {
    let (velocity, mut transform) = q_player.single_mut();

    let translation = &mut transform.translation;
    translation.x += velocity.x * TIME_STEP * BASE_SPEED;
    translation.y += velocity.y * TIME_STEP * BASE_SPEED;
}
