use bevy::prelude::*;

mod collision;
mod events;
mod input;
mod movement;
pub mod spawn;
pub mod spells;
mod sprite;
mod state;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::new(150., 150., 0.);
pub const PLAYER_SCALE: Vec3 = Vec3::splat(0.8);
pub const PLAYER_MAX_HEALTH: f32 = 3.;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn::PlayerSpawnPlugin)
            .add_plugins(movement::PlayerMovementPlugin)
            .add_plugins(input::PlayerInputPlugin)
            .add_plugins(sprite::PlayerSpritePlugin)
            .add_plugins(state::PlayerStatePlugin)
            .add_plugins(spells::PlayerSpellsPlugin)
            .add_plugins(collision::PlayerCollisionPlugin)
            .add_plugins(events::PlayerEventsPlugin);
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub collider_entity: Entity,
}
impl Player {
    fn new(collider_entity: Entity) -> Player {
        Player { collider_entity }
    }
}

#[derive(Component, Debug)]
pub struct Health {
    pub health: f32,
}

// events player can emit to be used on state transitions
#[derive(Event)]
pub enum PlayerEvents {
    GotHit(GotHitInfo),
}

pub struct GotHitInfo {
    damage: f32,
}
