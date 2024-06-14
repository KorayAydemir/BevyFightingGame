use bevy::prelude::*;

use super::{input::{PlayerInput, Direction}, spells::Spell, Player};


#[derive(States, Hash, Eq, PartialEq, Debug, Default, Clone)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving(Direction),
    CastingSpell(Spell),
}

fn switch_player_state(mut q_player: Query<&mut Player>, player_input: Res<PlayerInput>) {
    let mut player = q_player.single_mut();

    match player.state {
        PlayerState::Idling => {
            if let Some(direction) = player_input.move_direction { 
                player.state = PlayerState::Moving(direction);
            };

            if let Some(spell) = player_input.use_spell {
                player.state = PlayerState::CastingSpell(spell);
            };
        }

        PlayerState::Moving(_) => {
            if let Some(direction) = player_input.move_direction { 
                player.state = PlayerState::Moving(direction);
            };

            if let Some(spell) = player_input.use_spell {
                player.state = PlayerState::CastingSpell(spell);
            };

            if player_input.move_direction.is_none() {
                player.state = PlayerState::Idling;
            }
        }

        PlayerState::CastingSpell(spell) => {
            match spell {
                Spell::SprayFire => { }
                Spell::BlastWave => todo!(),
            }

            if let Some(direction) = player_input.move_direction {
                player.state = PlayerState::Moving(direction);
            } else {
                player.state = PlayerState::Idling;
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct PlayerChangedState {
    pub old_state: PlayerState,
    pub new_state: PlayerState,
}

fn player_changed_state(
    q_player: Query<&Player>,
    mut ev_changed_state: EventWriter<PlayerChangedState>,
    mut old_state: Local<PlayerState>,
) {
    let player = q_player.get_single().unwrap();

    if player.state != *old_state {
        println!(
            "Player state changed: {:?} -> {:?}",
            old_state, player.state
        );
        ev_changed_state.send(PlayerChangedState {
            old_state: old_state.clone(),
            new_state: player.state.clone(),
        });

        *old_state = player.state.clone();
    }
}

pub struct PlayerStatePlugin;
impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerChangedState>().add_systems(
            PostUpdate,
            (
                switch_player_state,
                player_changed_state.after(switch_player_state),
            ),
        );
    }
}
