use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Health, Player, PLAYER_MAX_HEALTH, PLAYER_SCALE, PLAYER_SPAWN_POS};
use crate::{common::sprite::AnimationTimer, GameState};

pub struct PlayerSpawnPlugin;
impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, set_velocity_to_zero)
            .add_systems(Update, player_death)
            .add_systems(Update, despawn_player);
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

fn player_death(
    q_player: Query<(Entity, &Health), With<Player>>,
    mut game_next_state: ResMut<NextState<GameState>>,
) {
    let (player_entity, health) = q_player.single();

    if health.health <= 2. {
        println!("Player died!");
        game_next_state.set(GameState::GameOver);
    }
}

fn despawn_player(
    mut commands: Commands,
    res_game_state: Res<State<GameState>>,
    q_player: Query<Entity, With<Player>>,
) {
    let player_entity = q_player.single();
    if res_game_state.is_changed() && *res_game_state.get() == GameState::GameOver {
        //commands.entity(player_entity).despawn_recursive();
    }
}
