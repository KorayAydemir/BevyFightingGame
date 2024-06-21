use bevy::prelude::*;

mod input;
mod movement;
pub mod spawn;
pub mod spells;
mod sprite;
mod state;
pub mod collision;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::new(-200., 0., 0.);
pub const PLAYER_SCALE: Vec3 = Vec3::splat(1.);
pub const PLAYER_MAX_HEALTH: f32 = 3.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn::PlayerSpawnPlugin)
            .add_plugins(movement::PlayerMovementPlugin)
            .add_plugins(input::PlayerInputPlugin)
            .add_plugins(sprite::PlayerSpritePlugin)
            .add_plugins(state::PlayerStatePlugin)
            .add_plugins(spells::PlayerSpellsPlugin)
            .add_plugins(collision::PlayerCollisionPlugin);
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub collider_entity: Entity
}

impl Player {
    fn new(collider_entity: Entity) -> Player {
        Player {
            collider_entity
        }
    }
}


#[derive(Component)]
pub struct Health {
    pub health: f32
}
