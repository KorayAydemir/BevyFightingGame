use bevy::{prelude::*, time::Time};

use super::PlayerSet;
use super::{state::PlayerState, Player};
use crate::common::sprite::flip_sprite;
use crate::common::sprite::update_spritesheet_indices;
use crate::common::sprite::AnimationTimer;

pub struct PlayerSpritePlugin;
impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_indices, flip_sprite::<Player, PlayerState>).in_set(PlayerSet));
    }
}

fn player_sprite_indicies(state: &PlayerState) -> (usize, usize) {
    match state {
        PlayerState::Idling => (8, 14),
        PlayerState::Moving(_) => (0, 7),
        PlayerState::CastingSpell(_) => (15, 18),
        PlayerState::Melee => (16, 19),
        PlayerState::Dead => (0, 0),
    }
}

fn update_indices(
    time: Res<Time>,
    mut q_player: Query<(&mut AnimationTimer, &mut TextureAtlas), With<Player>>,
    res_player_state: Res<State<PlayerState>>,
) {
    let (animation_timer, atlas) = q_player.get_single_mut().unwrap();
    let player_state = res_player_state.get();
    let indices = player_sprite_indicies(player_state);

    update_spritesheet_indices(&time, animation_timer, atlas, res_player_state, indices);
}
