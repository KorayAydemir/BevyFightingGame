#![warn(clippy::pedantic)]
#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::module_name_repetitions, clippy::needless_pass_by_value)]

mod player;

use std::env;

use bevy::{
    prelude::*,
    render::{camera::ScalingMode, render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
    window::{PresentMode, WindowMode},
};
use bevy_hanabi::HanabiPlugin;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

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
                })
                .set(RenderPlugin {
                    render_creation: wgpu_settings.into(),
                    synchronous_pipeline_compilation: false,
                }),
        )
        .add_plugins(HanabiPlugin)
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
