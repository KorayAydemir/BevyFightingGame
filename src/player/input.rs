use bevy::prelude::*;

use crate::GameState;

use super::spells::Spell;

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
            .add_systems(Update, use_spray_fire.run_if(in_state(GameState::Playing)))
            .add_systems(Update, use_melee.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Horizontal {
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Vertical {
    Up,
    Down,
}

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Option<Direction>,
    pub use_spell: Option<Spell>,
    pub use_melee: bool,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Direction {
    pub vertical: Option<Vertical>,
    pub horizontal: Option<Horizontal>,
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

fn use_spray_fire(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    if keys.just_pressed(KeyCode::KeyC) {
        player_input.use_spell = Some(Spell::SprayFire);
    } else {
        player_input.use_spell = None;
    }
}

fn use_melee(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    player_input.use_melee = keys.just_pressed(KeyCode::Space);
}
