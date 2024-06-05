use bevy::prelude::*;

use super::state::PlayerState;

fn player_sprite_indices(state: &PlayerState) {
    match state {
        PlayerState::Idling => (0, 3),
        PlayerState::Moving => (4, 12)
    };
}

pub struct PlayerSpritePlugin;
impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, update_indices);
    }
}
