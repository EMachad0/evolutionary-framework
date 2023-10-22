pub mod config;
pub mod textures;

use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((config::ConfigPlugin, textures::TexturesPlugin));
    }
}
