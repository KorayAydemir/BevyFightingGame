use bevy::prelude::*;
use bevy_math::Vec2;

use super::spells::{Spell, SpellDetails};

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
    pub use_spell: Option<Spell>,
}

fn player_movement(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    player_input.move_direction = direction;
}

fn use_spray_fire(keys: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    if keys.just_pressed(KeyCode::KeyC) {
        player_input.use_spell = Some(Spell::SprayFire);
    } else {
        player_input.use_spell = None;
    }
}

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInput>()
            .add_systems(Update, player_movement)
            .add_systems(Update, use_spray_fire);
    }
}
