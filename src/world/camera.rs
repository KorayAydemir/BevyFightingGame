use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{player::Player, GameState};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.,0.,0.)),
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::FixedVertical(400.0);
    commands.spawn(camera);
}

fn follow_player(
    q_player: Query<&Transform, With<Player>>,
    mut q_camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_translation = &q_player.single().translation;
    let camera_translation = &mut q_camera.single_mut().translation;

    camera_translation.x = player_translation.x;
    camera_translation.y = player_translation.y;
}
