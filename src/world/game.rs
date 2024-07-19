use bevy::prelude::*;

use crate::player::{state::PlayerState, Health, Player, PLAYER_MAX_HEALTH};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_event::<GameEvents>()
            .add_systems(Update, reset_game_state.run_if(on_event::<GameEvents>()))
            .add_systems(
                PostUpdate,
                log_game_state_transitions.run_if(state_changed::<GameState>),
            );
    }
}

fn log_game_state_transitions(mut ev_changed_state: EventReader<StateTransitionEvent<GameState>>) {
    for event in ev_changed_state.read() {
        println!(
            "Game state changed: {:?} -> {:?}",
            event.before, event.after
        );
    }
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Event)]
pub enum GameEvents {
    ResetGameState,
}

fn reset_game_state(
    mut ev_reset_game_state: EventReader<GameEvents>,
    mut game_next_state: ResMut<NextState<GameState>>,
    mut player_health: Query<&mut Health, With<Player>>,
    mut player_state: ResMut<NextState<PlayerState>>,
) {
    for _ in ev_reset_game_state.read() {
        println!("Resetting game state...");
        game_next_state.set(GameState::Playing);
        player_health.single_mut().health = PLAYER_MAX_HEALTH;
        player_state.set(PlayerState::Idling);
    }
}
