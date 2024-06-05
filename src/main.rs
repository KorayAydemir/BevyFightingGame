#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod systems {
    pub mod bloom;
    pub mod draw_cursor;
    pub mod pixel_grid_snap;
    pub mod shapes;
    pub mod sprite_animation;
    pub mod sprite_movement;
    pub mod volume_intersections_2d;
}
mod player;

use std::env;

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    window::{PresentMode, WindowMode},
};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Msaa::Off)
        .add_systems(Startup, spawn_camera)
        .add_plugins(player::PlayerPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::FixedVertical(400.0);

    commands.spawn(camera);
}
