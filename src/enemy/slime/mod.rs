use bevy::prelude::*;

mod spawn;

const SLIME_SPAWN_POS: Vec3 = Vec3::new(200., 0., 0.);

const SLIME_SCALE: Vec3 = Vec3::splat(1.);

pub struct SlimePlugin;
impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn::SlimeSpawnPlugin);
    }
}


#[derive(Component)]
pub struct Slime;
impl Slime {
    pub fn new() -> Slime {
        Slime {}
    }
}
