use crate::player::spawn::Velocity;
use bevy::prelude::*;

use super::Player;

fn player_movement(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut velocity = query.single_mut();

    velocity.x = 0.;
    velocity.y = 0.;

    if keys.pressed(KeyCode::KeyD) {
        velocity.x = 1.
    }

    if keys.pressed(KeyCode::KeyA) {
        velocity.x = -1.
    }

    if keys.pressed(KeyCode::KeyW) {
        velocity.y = 1.
    }

    if keys.pressed(KeyCode::KeyS) {
        velocity.y = -1.
    }
}

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement);
    }
}
