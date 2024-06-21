use bevy::prelude::*;

mod slime;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(slime::SlimePlugin);
    }
}

#[derive(Component)]
pub struct Enemy {
    pub damage: f32
}
