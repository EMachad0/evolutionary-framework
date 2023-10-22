use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use evolutionary_framework::assets::{toml_asset::TomlAsset, txt_asset::TxtAsset};
use evolutionary_framework::config::{parse_config, ConfigSet, ConfigToml, CONFIG_SCHEDULE};
use evolutionary_framework::GameState;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, ConfigAssets>(GameState::Loading)
            .add_systems(
                CONFIG_SCHEDULE,
                parse_config::<ConfigAssets>.in_set(ConfigSet),
            );
    }
}

#[derive(Clone, Resource, AssetCollection)]
pub struct ConfigAssets {
    #[asset(path = "configs/maze.toml")]
    pub config: Handle<TomlAsset>,
    #[asset(path = "files/maze.txt")]
    pub maze: Handle<TxtAsset>,
}

impl ConfigToml for ConfigAssets {
    fn handle(&self) -> &Handle<TomlAsset> {
        &self.config
    }
}
