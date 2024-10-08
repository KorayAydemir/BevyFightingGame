use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{
    state::PlayerState, Health, Player, PlayerSet, PLAYER_MAX_HEALTH, PLAYER_SCALE,
    PLAYER_SPAWN_POS,
};
use crate::{common::sprite::AnimationTimer, GameState};

pub struct PlayerSpawnPlugin;
impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_death.in_set(PlayerSet));
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::new(128, 128), 8, 3, Some(UVec2::new(16, 0)), None);
    let atlas_layout_handle = texture_atlas_layouts.add(atlas_layout);

    let player = commands
        .spawn((
            Name::new("Player"),
            RigidBody::Dynamic,
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
            SpriteSheetBundle {
                transform: Transform::from_translation(PLAYER_SPAWN_POS).with_scale(PLAYER_SCALE),
                texture: asset_server.load("player_spritesheet.png"),
                atlas: TextureAtlas {
                    layout: atlas_layout_handle,
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
    mut commands: Commands,
    q_player: Query<Entity, With<Player>>,
    player_state: Res<State<PlayerState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let player_entity = q_player.single();

    if let PlayerState::Dead = player_state.get() {
        println!("player died");
        //next_game_state.set(GameState::GameOver);
        //commands.entity(player_entity).despawn();
    }
}
