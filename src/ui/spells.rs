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
        let timer = cooldown_timers
            .0
            .get(spell)
            .expect("Cooldown timer for spell must exist");
        style.height =
            Val::Percent(timer.remaining_secs() * (100. / timer.duration().as_secs_f32()));
    }
}

fn spawn_cooldown_box_for_spell(
    container: &mut EntityCommands,
    spell_type: Spell,
    i: f32,
    asset_server: &Res<AssetServer>,
) {
    let spell_details = spell_type.details();

    container.with_children(|parent| {
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(60.0),
                        height: Val::Px(60.0),
                        left: Val::Px(210. + (i * 30.)),
                        ..default()
                    },
                    background_color: Color::rgb(0., 255., 0.).into(),
                    ..default()
                },
                Outline::new(Val::Px(5.), Val::ZERO, Color::GREEN),
            ))
            .with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.),
                            ..default()
                        },
                        background_color: Color::rgba(0., 0., 0., 0.9).into(),
                        z_index: ZIndex::Global(20),
                        ..default()
                    },
                    spell_type,
                    SpellBox,
                ));
            })
            .with_children(|parent| {
                parent
                    .spawn((NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        z_index: ZIndex::Global(10),
                        background_color: Color::rgb(0., 0., 255.).into(),
                        ..default()
                    },))
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: UiImage::new(asset_server.load(spell_details.ui_icon)),
                            ..default()
                        });
                    });
            });
    });
}

fn spawn_cooldown_box(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut cooldown_box_container = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(80.0),
            height: Val::Percent(10.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.),
            ..default()
        },
        ..default()
    });

    for (i, spell) in Spell::VALUES.iter().enumerate() {
        spawn_cooldown_box_for_spell(&mut cooldown_box_container, *spell, i as f32, &asset_server);
    }
}
