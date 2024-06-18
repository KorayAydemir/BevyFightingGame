use bevy::prelude::*;

use super::{
    input::{Direction, PlayerInput},
    spells::Spell,
};

pub struct PlayerStatePlugin;
impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .insert_resource(Melee {
                cooldown_timer: Timer::from_seconds(3.0, TimerMode::Once),
                swinging_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            })
            .add_systems(PostUpdate, switch_player_state)
            .add_systems(
                PostUpdate,
                log_player_state_transitions.after(switch_player_state),
            )
            .add_systems(PostUpdate, melee_cooldown);
    }
}

#[derive(Resource)]
struct Melee {
    cooldown_timer: Timer,
    swinging_timer: Timer,
}

fn melee_cooldown(
    mut melee: ResMut<Melee>,
    time: Res<Time>,
    player_state: Res<State<PlayerState>>,
) {
    melee.cooldown_timer.tick(time.delta());
    if *player_state.get() == PlayerState::Melee {
        melee.swinging_timer.tick(time.delta());
    }
}

pub enum EntityState {
    Player(PlayerState),
}

#[derive(States, Hash, Eq, PartialEq, Debug, Default, Clone)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving(Direction),
    CastingSpell(Spell),
    Melee,
}

fn switch_player_state(
    player_state: Res<State<PlayerState>>,
    mut player_next_state: ResMut<NextState<PlayerState>>,
    player_input: Res<PlayerInput>,
    mut res_melee: ResMut<Melee>,
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

            if player_input.use_melee && res_melee.cooldown_timer.finished() {
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

            if player_input.use_melee && res_melee.cooldown_timer.finished() {
                player_next_state.set(PlayerState::Melee);
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

        PlayerState::Melee => {
            if res_melee.swinging_timer.finished() {
                player_next_state.set(PlayerState::Idling);

                res_melee.cooldown_timer.reset();
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
