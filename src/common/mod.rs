use bevy::prelude::*;

mod components;
pub mod sprite;
pub mod movement;

pub struct CommonPlugin;
impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
    }
}
