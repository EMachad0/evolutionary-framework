pub mod toml_asset;
pub mod txt_asset;

use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((toml_asset::TomlAssetPlugin, txt_asset::TxtAssetPlugin));
    }
}
