use bevy::{prelude::*, time::Time};

use super::{input::Horizontal, state::PlayerState, Player};
use crate::common::sprite::update_spritesheet_indices;
use crate::common::sprite::AnimationTimer;
use crate::GameState;

pub struct PlayerSpritePlugin;
impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_indices.run_if(in_state(GameState::Playing)))
            .add_systems(Update, flip_sprite.run_if(in_state(GameState::Playing)));
    }
}

fn player_sprite_indicies(state: &PlayerState) -> (usize, usize) {
    match state {
        PlayerState::Idling => (8, 14),
        PlayerState::Moving(_) => (0, 7),
        PlayerState::CastingSpell(_) => (15, 18),
        PlayerState::Melee => (16, 19),
    }
}

fn update_indices(
    time: Res<Time>,
    mut q_player_anim: Query<(&mut AnimationTimer, &mut TextureAtlas), With<Player>>,
    res_player_state: Res<State<PlayerState>>,
) {
    let (animation_timer, atlas) = q_player_anim.get_single_mut().unwrap();
    let player_state = res_player_state.get();
    let indices = player_sprite_indicies(player_state);

    update_spritesheet_indices(&time, animation_timer, atlas, res_player_state, indices);
}

fn flip_sprite(
    mut q_player_sprite: Query<&mut Sprite, With<Player>>,
    player_state: Res<State<PlayerState>>,
) {
    let mut player_sprite = q_player_sprite.get_single_mut().unwrap();
    let player_state = player_state.get();

    if let PlayerState::Moving(direction) = player_state {
        if let Some(Horizontal::Left) = direction.horizontal {
            player_sprite.flip_x = true;
        }
    }

    if let PlayerState::Moving(direction) = player_state {
        if let Some(Horizontal::Right) = direction.horizontal {
            player_sprite.flip_x = false;
        }
    }
}
