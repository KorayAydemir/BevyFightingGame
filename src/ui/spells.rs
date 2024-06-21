use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::player::spells::{CooldownTimers, Spell};

pub struct SpellsUiPlugin;
impl Plugin for SpellsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cooldown_box);
        app.add_systems(Update, update_cooldown_box);
    }
}

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
                } else {
                    Val::Percent(0.)
                }
            }
            Spell::BlastWave => Val::Percent(0.),
            Spell::Melee => {
                if let Some(timer) = cooldown_timers.0.get(&Spell::Melee) {
                    Val::Percent(timer.elapsed_secs() * 50.)
                } else {
                    Val::Percent(0.)
                }
            }
        }
    }
}

fn spawn_cooldown_boxes_container<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    })
}

fn spawn_cooldown_box_for_spell(container: &mut EntityCommands, spell_type: Spell, i: f32) {
    container.with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(210. + (i * 30.)),
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
                    spell_type,
                    SpellBox,
                ));
            });
    });
}

fn spawn_cooldown_box(mut commands: Commands) {
    let mut container = spawn_cooldown_boxes_container(&mut commands);

    for (i, spell) in Spell::VALUES.iter().enumerate() {
        spawn_cooldown_box_for_spell(&mut container, *spell, i as f32);
    }
}
