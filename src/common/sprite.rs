use bevy::prelude::*;

use crate::common::movement::{Direction, Horizontal};
use crate::common::movement::CanMove;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn update_spritesheet_indices(
    time: &Res<Time>,
    mut timer: Mut<AnimationTimer>,
    mut atlas: Mut<TextureAtlas>,
    res_entity_state: impl DetectChanges,
    indices: (usize, usize),
) {
    if res_entity_state.is_changed() {
        atlas.index = indices.0;
    }

    timer.tick(time.delta());

    if timer.just_finished() {
        if atlas.index < indices.0 {
            atlas.index = indices.0;
        }

        if atlas.index == indices.1 {
            atlas.index = indices.0;
        } else {
            atlas.index += 1;
        }
    }
}


pub fn flip_sprite<Character: Component, CharacterState: CanMove + States>(
    mut q_char_sprite: Query<&mut Sprite, With<Character>>,
    char_state: Res<State<CharacterState>>,
) {
    let char_state = char_state.get();

    for mut sprite in &mut q_char_sprite {
        if let Some(Direction {
            horizontal: Some(Horizontal::Left),
            vertical: _,
        }) = char_state.get_move_direction()
        {
            sprite.flip_x = true;
        }

        if let Some(Direction {
            horizontal: Some(Horizontal::Right),
            vertical: _,
        }) = char_state.get_move_direction()
        {
            sprite.flip_x = false;
        }

        //if let CharacterState::Moving(direction) = char_state {
        //    if let Some(Horizontal::Left) = direction.horizontal {
        //        sprite.flip_x = true;
        //    }
        //}

        //if let CharacterState::Moving(direction) = char_state {
        //    if let Some(Horizontal::Right) = direction.horizontal {
        //        sprite.flip_x = false;
        //    }
        //}
    }
}
