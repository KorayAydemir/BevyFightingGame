use bevy::prelude::*;

mod camera;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin);
    }
}
