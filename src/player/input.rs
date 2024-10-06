use bevy::prelude::*;

use super::{spells::Spell, PlayerSet};
use crate::common::movement::{Direction, Horizontal, Vertical};

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>().add_systems(
            Update,
            (player_movement, use_spell).in_set(PlayerSet),
        );
    }
}

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Option<Direction>,
    pub use_spell: Option<Spell>,
}

fn player_movement(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let horizontal = if keys.pressed(KeyCode::KeyD) {
        Some(Horizontal::Right)
    } else if keys.pressed(KeyCode::KeyA) {
        Some(Horizontal::Left)
    } else {
        None
    };

    let vertical = if keys.pressed(KeyCode::KeyW) {
        Some(Vertical::Up)
    } else if keys.pressed(KeyCode::KeyS) {
        Some(Vertical::Down)
    } else {
        None
    };

    if horizontal.is_some() || vertical.is_some() {
        player_input.move_direction = Some(Direction {
            vertical,
            horizontal,
        });
    } else {
        player_input.move_direction = None;
    }
}

fn use_spell(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    if keys.just_pressed(KeyCode::KeyV) {
        player_input.use_spell = Some(Spell::BlazingSword);
    } else if keys.just_pressed(KeyCode::KeyC) {
        player_input.use_spell = Some(Spell::SprayFire);
    } else if keys.just_pressed(KeyCode::Space) {
        player_input.use_spell = Some(Spell::Melee);
    } else {
        player_input.use_spell = None;
    }
}
