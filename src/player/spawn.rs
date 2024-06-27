use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::{Health, Player, PLAYER_MAX_HEALTH, PLAYER_SCALE, PLAYER_SPAWN_POS};
use crate::common::sprite::AnimationTimer;

pub struct PlayerSpawnPlugin;
impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(Update, set_velocity_to_zero);
    }
}

fn set_velocity_to_zero(mut q_player: Query<&mut Velocity, With<Player>>) {
    let mut player_velocity = q_player.single_mut();
    player_velocity.linvel = Vec2::ZERO;
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(128., 128.), 8, 3, Some(Vec2::new(16., 0.)), None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player = commands
        .spawn((
            //RigidBody::KinematicPositionBased,
            RigidBody::Dynamic,
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
            SpriteSheetBundle {
                transform: Transform::from_translation(PLAYER_SPAWN_POS).with_scale(PLAYER_SCALE),
                texture: asset_server.load("player_spritesheet.png"),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            Health {
                health: PLAYER_MAX_HEALTH,
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::cuboid(8.0, 25.),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., -25., 0.))),
            ActiveEvents::COLLISION_EVENTS,
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        ))
        .id();

    commands
        .entity(player)
        .insert(Player::new(collider))
        .push_children(&[collider]);
}
