use bevy::prelude::*;

use crate::player::spells::{CooldownTimers, Spell};

#[derive(Component)]
pub struct SpellBox;

fn update_cooldown_box(
    mut q_cooldown_box: Query<(&mut Style, &Spell), With<SpellBox>>,
    cooldown_timers: Res<CooldownTimers>,
) {
    for (mut style, spell) in &mut q_cooldown_box {
        style.height = match spell {
            Spell::SprayFire => {
                if let Some(timer) = cooldown_timers.0.get(&Spell::SprayFire) {
                    Val::Percent(timer.elapsed_secs() * 50.)

                    //if timer.finished() {
                    // flash the spellbox
                    //} else {
                    //}
                } else {
                    Val::Percent(0.)
                }
            }
            Spell::BlastWave => {
                todo!()
            }
        }
    }
}

fn spawn_cooldown_box(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(210.),
                        bottom: Val::Px(10.),
                        border: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    border_color: Color::GREEN.into(),
                    background_color: Color::rgb(0., 0., 0.).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::rgb(1., 1., 1.).into(),
                            ..default()
                        },
                        Spell::SprayFire,
                        SpellBox,
                    ));
                });
        });
}

pub struct SpellsUiPlugin;

impl Plugin for SpellsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cooldown_box);
        app.add_systems(Update, update_cooldown_box);
    }
}
