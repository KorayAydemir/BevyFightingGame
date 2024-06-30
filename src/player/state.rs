use bevy::prelude::*;

use super::{
    input::PlayerInput,
    spells::{CastingTimers, CooldownTimers, PlayerMeleeHitbox, Spell},
};

use crate::{common::movement::Direction, impl_can_move, GameState};

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
    Melee,
    Dead,
}
impl_can_move!(PlayerState);

fn switch_player_state(
    mut commands: Commands,
    player_state: Res<State<PlayerState>>,
    mut player_next_state: ResMut<NextState<PlayerState>>,
    player_input: Res<PlayerInput>,
    casting_timers: Res<CastingTimers>,
    cd_timers: Res<CooldownTimers>,
    q_entity_melee_hitbox: Query<Entity, With<PlayerMeleeHitbox>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let player_state = player_state.get();

    match player_state {
        PlayerState::Idling => {
            if let Some(direction) = player_input.move_direction {
                player_next_state.set(PlayerState::Moving(direction));
            };

            if let Some(spell) = player_input.use_spell {
                player_next_state.set(PlayerState::CastingSpell(spell));
            };

            if player_input.use_melee && !is_melee_in_cooldown(cd_timers) {
                player_next_state.set(PlayerState::Melee);
            }
        }

        PlayerState::Moving(_) => {
            if let Some(direction) = player_input.move_direction {
                player_next_state.set(PlayerState::Moving(direction));
            };

            if let Some(spell) = player_input.use_spell {
                player_next_state.set(PlayerState::CastingSpell(spell));
            };

            if player_input.move_direction.is_none() {
                player_next_state.set(PlayerState::Idling);
            }

            if player_input.use_melee && !is_melee_in_cooldown(cd_timers) {
                player_next_state.set(PlayerState::Melee);
            }
        }

        PlayerState::CastingSpell(spell) => {
            match spell {
                Spell::SprayFire => {}
                Spell::BlazingSword => {}
                Spell::Melee => {}
            }

            if let Some(direction) = player_input.move_direction {
                player_next_state.set(PlayerState::Moving(direction));
            } else {
                player_next_state.set(PlayerState::Idling);
            }
        }

        PlayerState::Melee => {
            if let Some(casting) = casting_timers.0.get(&Spell::Melee) {
                if !casting.finished() {
                    return;
                }

                if casting.finished() {
                    let entity_melee_hitbox = q_entity_melee_hitbox.single();
                    commands.entity(entity_melee_hitbox).despawn();
                }
            }

            if let Some(direction) = player_input.move_direction {
                player_next_state.set(PlayerState::Moving(direction));
            } else {
                player_next_state.set(PlayerState::Idling);
            }
        }
        PlayerState::Dead => {
            next_game_state.set(GameState::GameOver);
        }
    }
}

fn is_melee_in_cooldown(cooldown_timers: Res<CooldownTimers>) -> bool {
    if let Some(cooldown) = cooldown_timers.0.get(&Spell::Melee) {
        !cooldown.finished()
    } else {
        false
    }
}

fn log_player_state_transitions(
    mut ev_changed_state: EventReader<StateTransitionEvent<PlayerState>>,
) {
    for event in ev_changed_state.read() {
        println!(
            "Player state changed: {:?} -> {:?}",
            event.before, event.after
        );
    }
}
