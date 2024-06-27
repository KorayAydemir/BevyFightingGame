use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_systems(Startup, spawn_map_borders)
            .add_systems(Startup, spawn_tilemap)
            .insert_resource(LevelSelection::index(0));
    }
}

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemap/hills.ldtk"),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -5.0)),
        ..Default::default()
    });
}

fn spawn_map_borders(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {

    rapier_config.gravity = Vec2::ZERO;

    commands.spawn((
        Collider::cuboid(10.0, 2600.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(10.0, 2600.0, 0.0))),
        RigidBody::KinematicPositionBased,
        ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
    ));
    commands.spawn((
        Collider::cuboid(10.0, 2600.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            5120.0, 2600.0, 0.0,
        ))),
    ));
    commands.spawn((
        Collider::cuboid(2600.0, 10.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(2610.0, 25.0, 0.0))),
    ));
    commands.spawn((
        Collider::cuboid(2600.0, 10.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            2610.0, 5120.0, 0.0,
        ))),
    ));
}
