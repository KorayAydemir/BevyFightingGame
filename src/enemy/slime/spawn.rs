use bevy::prelude::*;

use super::{Slime, SLIME_SCALE, SLIME_SPAWN_POS};
use crate::common::components::AnimationTimer;


fn spawn_slime(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(128., 128.), 8, 3, None, None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_translation(SLIME_SPAWN_POS).with_scale(SLIME_SCALE),
            texture: asset_server.load("slime_blue.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            ..default()
        },
        Slime::new(),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}

pub struct SlimeSpawnPlugin;
impl Plugin for SlimeSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_slime);
    }
}
