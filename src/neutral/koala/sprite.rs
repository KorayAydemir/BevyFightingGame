use bevy::prelude::*;

use crate::common::sprite::flip_sprite;

use super::{state::KoalaState, Koala};

pub struct KoalaSpritePlugin;
impl Plugin for KoalaSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flip_sprite::<Koala, KoalaState>);
    }
}
