// assets from https://craftpix.net/
#![warn(clippy::pedantic)]
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value
)]

use std::env;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_toggle_active, prelude::*, render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin}, window::{PresentMode, WindowMode, WindowResolution}
};
use bevy_hanabi::HanabiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};
use player::{spells::PlayerSpellsSet, PlayerSet};

mod common;
mod enemy;
mod player;
mod ui;
mod world;
mod neutral;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

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
                        resolution: WindowResolution::new(1280., 720.),
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
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(PerfUiPlugin)
        .add_plugins(HanabiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F10)))
        .add_systems(Startup, spawn_perfui)

        .init_state::<GameState>()

        .add_plugins(world::WorldPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(common::CommonPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(neutral::NeutralPlugin)

        .configure_sets(
            Update,
            PlayerSet.run_if(in_state(GameState::Playing)),
        )

        .run();
}

fn spawn_perfui(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}
