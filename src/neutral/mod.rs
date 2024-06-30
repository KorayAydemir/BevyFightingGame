use bevy::prelude::*;

mod koala;

pub struct NeutralPlugin;
impl Plugin for NeutralPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(koala::NeutralKoalaPlugin);
    }
}
