use bevy::prelude::*;

mod camera;
mod map;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin)
            .add_plugins(map::MapPlugin);
    }
}
