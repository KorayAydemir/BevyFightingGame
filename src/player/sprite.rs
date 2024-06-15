use bevy::prelude::*;

use super::{input::Horizontal, state::PlayerState, Player};

fn player_sprite_indices(state: &PlayerState) {
    match state {
        PlayerState::Idling => (0, 3),
        PlayerState::Moving(_) => (4, 12),
        PlayerState::CastingSpell(_) => (0,0)
    };
}

pub struct PlayerSpritePlugin;
impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flip_sprite);
    }
}

fn flip_sprite(mut q_player_sprite: Query<&mut Sprite, With<Player>>, player_state: Res<State<PlayerState>>) {
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
