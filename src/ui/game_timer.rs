use bevy::{prelude::*, time::Stopwatch};

use crate::world::game::GameState;

pub struct GameTimer;
impl Plugin for GameTimer {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameDuration(Stopwatch::new()))
            .add_systems(
                Update,
                update_game_timer.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Startup, spawn_game_timer);
    }
}

#[derive(Resource)]
pub struct GameDuration(pub Stopwatch);

#[derive(Component)]
struct GameDurationText;

fn update_game_timer(
    time: Res<Time>,
    mut game_duration: ResMut<GameDuration>,
    mut query: Query<(&GameDurationText, &mut Text)>,
) {
    game_duration.0.tick(time.delta());

    for (_, mut text) in &mut query {
        text.sections[0].value = format!("{:.1}", game_duration.0.elapsed().as_secs_f32());
    }
}

fn spawn_game_timer(mut commands: Commands) {
    commands.spawn((
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            text: Text::from_section(
                "0.0",
                TextStyle {
                    font_size: 40.0,
                    ..default()
                },
            ),
            ..default()
        },
        GameDurationText,
    ));
}
