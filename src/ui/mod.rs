use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_perf_ui::ui::root::PerfUiRoot;
use iyes_perf_ui::{
    entries::{
        diagnostics::{PerfUiEntryFPS, PerfUiEntryFPSWorst},
        window::PerfUiEntryWindowResolution,
    },
    PerfUiPlugin,
};

pub mod game_timer;
mod health;
mod main_menu;
mod spells;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_toggle_active};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_plugins(
                WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F10)),
            )
            .add_plugins(spells::SpellsUiPlugin)
            .add_plugins(health::HealthUiPlugin)
            .add_plugins(main_menu::MainMenuPlugin)
            .add_plugins(game_timer::GameTimer)
            .add_systems(Startup, spawn_perfui);
    }
}

fn spawn_perfui(mut commands: Commands) {
    //commands.spawn(PerfUiCompleteBundle::default());
    //commands.spawn((
    //    PerfUiRoot {
    //        position: iyes_perf_ui::PerfUiPosition::BottomRight,
    //        ..default()
    //    },
    //    PerfUiEntryFPS::default(),
    //    PerfUiEntryFPSWorst::default(),
    //    PerfUiEntryWindowResolution::default(),
    //));
}
