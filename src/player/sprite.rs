use bevy::prelude::*;

use super::{input::Horizontal, spawn::AnimationTimer, state::PlayerState, Player};

fn player_sprite_indicies(state: &PlayerState) -> (usize, usize) {
    match state {
        PlayerState::Idling => (8, 14),
        PlayerState::Moving(_) => (0, 7),
        PlayerState::CastingSpell(_) => (13, 15),
    }
}

// todo: more generalized function for animating all sprites in the world
//fn animate_sprite(
//    time: Res<Time>,
//    mut query: Query<(
//        &mut AnimationIndices,
//        &mut AnimationTimer,
//        &mut TextureAtlas,
//    )>,
//) {
//    for (mut indices, mut timer, mut atlas) in &mut query {
//        timer.tick(time.delta());
//
//        if timer.just_finished() {
//        }
//    }
//}

fn update_indices(
    time: Res<Time>,
    mut q_player_anim: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
    res_player_state: Res<State<PlayerState>>,
    mut prev_anim_indices: Local<(usize, usize)>,
) {
    let (mut timer, mut atlas) = q_player_anim.single_mut();

    let player_state = res_player_state.get();
    let anim_indices = player_sprite_indicies(player_state);

    if res_player_state.is_changed() {
        atlas.index = anim_indices.0;
        *prev_anim_indices = anim_indices;
    }

    timer.tick(time.delta());

    if timer.just_finished() {
        if atlas.index < anim_indices.0 {
            atlas.index = anim_indices.0;
        }

        println!("atlas index {}", atlas.index);
        println!("anim ind {}", anim_indices.1);
        if atlas.index == anim_indices.1 {
            atlas.index = anim_indices.0;
        } else {
            atlas.index += 1;
        }
    }
}

pub struct PlayerSpritePlugin;
impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_indices)
            .add_systems(Update, flip_sprite);
    }
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
