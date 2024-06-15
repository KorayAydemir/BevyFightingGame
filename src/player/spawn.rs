use bevy::prelude::*;

use super::{Player, PLAYER_SCALE, PLAYER_SPAWN_POS};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(128., 128.), 8, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
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
    ));
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
