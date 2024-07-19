use bevy::prelude::*;

mod collision;
mod movement;
mod spawn;
mod sprite;
mod state;

pub const MIN_RANDOM_SLIME_SCALE: f32 = 1.0;
pub const MAX_RANDOM_SLIME_SCALE: f32 = 2.0;
pub const MEGA_SLIME_SCALE: f32 = 5.0;
pub const SLIME_BASE_SPEED: f32 = 10.;

pub struct SlimePlugin;
impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn::SlimeSpawnPlugin)
            .add_plugins(movement::SlimeMovementPlugin)
            .add_plugins(state::SlimeStatePlugin)
            .add_plugins(sprite::SlimeSpritePlugin)
            .add_plugins(collision::SlimeCollisionPlugin);
    }
}

#[derive(Component)]
pub struct Slime {
    id: usize,
}

impl Slime {
    pub fn new(id: usize) -> Slime {
        Slime { id }
    }
}
