use bevy::prelude::*;

use super::Player;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving,
    CastingSpell,
}

fn switch_player_state(keys: Res<ButtonInput<KeyCode>>, mut q_player: Query<&mut Player>) {
    let mut player = q_player.single_mut();

    match player.state {
        PlayerState::Idling => {
            if keys.pressed(KeyCode::KeyC) {
                player.state = PlayerState::CastingSpell;
            }
        }
        PlayerState::Moving => {}

        PlayerState::CastingSpell => {}
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
        app.add_event::<PlayerChangedState>()
            .add_systems(PostUpdate, (switch_player_state, player_changed_state.after(switch_player_state)));
    }
}
