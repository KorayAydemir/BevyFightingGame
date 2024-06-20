use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Player, PLAYER_SCALE, PLAYER_SPAWN_POS};
use crate::common::sprite::AnimationTimer;

pub struct PlayerSpawnPlugin;
impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(96., 128.), 8, 3, Some(Vec2::new(32., 0.)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player = commands
        .spawn((
            SpriteSheetBundle {
                transform: Transform::from_translation(PLAYER_SPAWN_POS).with_scale(PLAYER_SCALE),
                texture: asset_server.load("player_spritesheet.png"),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
                ..default()
            },
            Player::new(),
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::cuboid(8.0, 25.),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., -25., 0.))),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .id();

    commands.entity(player).push_children(&[collider]);
}
