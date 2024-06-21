use bevy::prelude::*;
mod health;
mod spells;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spells::SpellsUiPlugin)
            .add_plugins(health::HealthUiPlugin);
    }
}
