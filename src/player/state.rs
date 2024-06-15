use bevy::prelude::*;

use super::{
    input::{Direction, PlayerInput},
    spells::Spell,
};

#[derive(States, Hash, Eq, PartialEq, Debug, Default, Clone)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving(Direction),
    CastingSpell(Spell),
}

fn switch_player_state(
    player_state: Res<State<PlayerState>>,
    mut player_next_state: ResMut<NextState<PlayerState>>,
    player_input: Res<PlayerInput>,
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
        }

        PlayerState::CastingSpell(spell) => {
            match spell {
                Spell::SprayFire => {}
                Spell::BlastWave => todo!(),
            }

            if let Some(direction) = player_input.move_direction {
                player_next_state.set(PlayerState::Moving(direction));
            } else {
                player_next_state.set(PlayerState::Idling);
            }
        }
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

pub struct PlayerStatePlugin;
impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_systems(PostUpdate, switch_player_state)
            .add_systems(PostUpdate, log_player_state_transitions.after(switch_player_state));
    }
}
