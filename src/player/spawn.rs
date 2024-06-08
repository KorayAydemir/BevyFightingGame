use bevy::prelude::*;

use super::{Player, PLAYER_SCALE, PLAYER_SPAWN_POS};

struct AnimationRange {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct AnimationIndices {
    idle: AnimationRange,
    go_rev: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64., 64.), 2, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices {
        idle: AnimationRange { first: 0, last: 3 },
        go_rev: 0,
    };

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_translation(PLAYER_SPAWN_POS).with_scale(PLAYER_SCALE),
            texture: asset_server.load("char1_idle_anim.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            ..default()
        },
        Player::new(),
        Velocity { x: 0., y: 0. },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
) {
    for (mut indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if atlas.index == indices.idle.last {
                indices.go_rev = indices.idle.last;
            } else if atlas.index == indices.idle.first {
                indices.go_rev = 0;
            }

            if indices.go_rev > 0 {
                atlas.index -= 1;
            } else {
                atlas.index += 1;
            }
        }
    }
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, animate_sprite);
    }
}
