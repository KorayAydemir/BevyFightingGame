use bevy::prelude::*;

mod movement;
mod spawn;
mod state;
mod sprite;
mod collision;

pub const MIN_SLIME_SCALE: f32 = 1.0;
pub const MAX_SLIME_SCALE: f32 = 1.4;

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
        Slime {
            id,
        }
    }
}
