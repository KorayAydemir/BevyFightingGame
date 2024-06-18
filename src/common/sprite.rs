use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn update_spritesheet_indices(
    time: Res<Time>,
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

        println!("atlas index {}", atlas.index);
        println!("anim ind {}", indices.1);
        if atlas.index == indices.1 {
            atlas.index = indices.0;
        } else {
            atlas.index += 1;
        }
    }
}
