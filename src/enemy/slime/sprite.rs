use bevy::prelude::*;

use crate::common::sprite::{update_spritesheet_indices, AnimationTimer};

use super::{state::SlimeState, Slime};

pub struct SlimeSpritePlugin;
impl Plugin for SlimeSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_indices);
    }
}

fn slime_sprite_indices(state: SlimeState) -> (usize, usize) {
    match state {
        SlimeState::Moving => (0, 0),
        SlimeState::Patrolling => (0, 3),
        SlimeState::Dead => (0, 0),
    }
}

fn update_indices(
    time: Res<Time>,
    mut q_slime: Query<(&mut AnimationTimer, &mut TextureAtlas, Ref<SlimeState>), With<Slime>>,
) {
    for (animation_timer, atlas, state) in &mut q_slime {
        let indices = slime_sprite_indices(*state);
        update_spritesheet_indices(&time, animation_timer, atlas, state, indices);
    }
}
