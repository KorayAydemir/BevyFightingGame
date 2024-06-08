use bevy::prelude::*;

use self::state::PlayerState;

mod input;
mod movement;
mod spawn;
mod sprite;
mod state;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::new(-200., 0., 0.);
pub const PLAYER_SCALE: Vec3 = Vec3::splat(1.);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn::PlayerSpawnPlugin)
            .add_plugins(movement::PlayerMovementPlugin)
            .add_plugins(input::PlayerInputPlugin)
            .add_plugins(sprite::PlayerSpritePlugin)
            .add_plugins(state::PlayerStatePlugin);
    }
}


#[derive(Component, Debug)]
pub struct Player {
    state: PlayerState
}

impl Player {
    fn new() -> Player {
        Player {
            state: PlayerState::default()
        }
    }
}
