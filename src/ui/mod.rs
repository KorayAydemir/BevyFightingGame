use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};
mod health;
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
            .add_systems(Startup, spawn_perfui);
    }
}

fn spawn_perfui(mut commands: Commands) {
    commands.spawn(PerfUiCompleteBundle::default());
}
