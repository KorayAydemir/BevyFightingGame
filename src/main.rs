#![warn(clippy::pedantic)]
#![allow(
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value
)]

use std::env;

use crate::world::game::GameState;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
};
use bevy_hanabi::HanabiPlugin;
use player::PlayerSet;

mod common;
mod enemy;
mod neutral;
mod player;
mod ui;
mod world;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // prevents blurry spots
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        resolution: WindowResolution::new(1280., 720.),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Msaa::Off)
        .add_plugins(HanabiPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(common::CommonPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(neutral::NeutralPlugin)
        .configure_sets(Update, PlayerSet.run_if(in_state(GameState::Playing)))
        .run();
}
