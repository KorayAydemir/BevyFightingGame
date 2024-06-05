use bevy::prelude::*;

#[derive(Debug, Default)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving,
}

pub struct PlayerStatePlugin;
impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
    }
}
