use bevy::prelude::*;

use crate::impl_can_move;

use super::{
    input::PlayerInput,
    spells::{CastingTimers, CooldownTimers, Spell},
};
use crate::common::movement::Direction;
use crate::world::game::GameState;

pub struct PlayerStatePlugin;
impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_systems(PostUpdate, switch_player_state)
            .add_systems(
                PostUpdate,
                log_player_state_transitions.after(switch_player_state),
            );
    }
}

#[derive(States, Hash, Eq, PartialEq, Debug, Default, Clone)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving(Direction),
    CastingSpell(Spell),
    Dead,
}

impl_can_move!(PlayerState);

fn switch_player_state(
    player_state: Res<State<PlayerState>>,
    mut player_next_state: ResMut<NextState<PlayerState>>,
    player_input: Res<PlayerInput>,
    casting_timers: Res<CastingTimers>,
    cooldown_timers: Res<CooldownTimers>,
) {
    let player_state = player_state.get();

    match player_state {
        PlayerState::Idling => {
            if let Some(direction) = player_input.move_direction {
                player_next_state.set(PlayerState::Moving(direction));
            };

            if let Some(spell) = player_input.use_spell {
                if spell.is_cooldown_finished(&cooldown_timers) {
                    player_next_state.set(PlayerState::CastingSpell(spell));
                }
            };
        }

        PlayerState::Moving(_) => {
            if let Some(direction) = player_input.move_direction {
                player_next_state.set(PlayerState::Moving(direction));
            };

            if let Some(spell) = player_input.use_spell {
                if spell.is_cooldown_finished(&cooldown_timers) {
                    player_next_state.set(PlayerState::CastingSpell(spell));
                }
            };

            if player_input.move_direction.is_none() {
                player_next_state.set(PlayerState::Idling);
            }
        }

        PlayerState::CastingSpell(spell) => {
            match spell {
                Spell::SprayFire | Spell::BlazingSword => {}
                Spell::Melee => {
                    if !Spell::Melee.is_casting_finished(&casting_timers) {
                        return;
                    }
                }
            }

            if let Some(direction) = player_input.move_direction {
                player_next_state.set(PlayerState::Moving(direction));
            } else {
                player_next_state.set(PlayerState::Idling);
            }
        }

        PlayerState::Dead => {
            println!("Player is dead!");
        }
    }
}

fn log_player_state_transitions(
    mut ev_changed_state: EventReader<StateTransitionEvent<PlayerState>>,
) {
    for event in ev_changed_state.read() {
        debug!(
            "Player state changed: {:?} -> {:?}",
            event.exited, event.entered
        )
        //println!(
        //    "Player state changed: {:?} -> {:?}",
        //    event.before, event.after
        //);
    }
}
