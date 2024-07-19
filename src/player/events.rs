use bevy::prelude::*;

use crate::world::game::GameState;

use super::{state::PlayerState, Health, Player, PlayerEvents, PlayerSet};

pub struct PlayerEventsPlugin;

impl Plugin for PlayerEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEvents>()
            .add_systems(PreUpdate, handle_events.in_set(PlayerSet));
    }
}

fn handle_events(
    mut player_events: EventReader<PlayerEvents>,
    mut q_player: Query<&mut Health, With<Player>>,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut player_health = q_player.single_mut();
    for ev in player_events.read() {
        match ev {
            PlayerEvents::GotHit(hit_info) => {
                player_health.health -= hit_info.damage;

                if player_health.health <= 0. {
                    player_state.set(PlayerState::Dead);
                    game_state.set(GameState::GameOver);
                }
            }
            PlayerEvents::KilledSlime(points) => {
                player_health.health += *points as f32 / 2.;
            }
        }
    }
}
