use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_cursor(camera_query: Query<(&Camera, &GlobalTransform)>, windows: Query<&Window>, mut gizmos: Gizmos) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::RED);

}

pub struct DrawCursorPlugin;
impl Plugin for DrawCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, draw_cursor);
    }
}
