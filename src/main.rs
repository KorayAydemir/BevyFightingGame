// assets from https://craftpix.net/
#![warn(clippy::pedantic)]
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value
)]

mod player;
mod ui;
mod world;

use std::env;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::{
        render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin,
    },
    window::{PresentMode, WindowMode},
};
use bevy_hanabi::HanabiPlugin;
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};

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
        .insert_resource(Msaa::Off)

        .add_plugins(PerfUiPlugin)
        .add_plugins(HanabiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)

        .add_plugins(world::WorldPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(ui::UiPlugin)

        .add_systems(Startup, spawn_perfui)
        .run();
}

fn spawn_perfui(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}
