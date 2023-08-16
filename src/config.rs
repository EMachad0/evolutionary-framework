use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use serde::{Deserialize, Serialize};

use crate::toml_asset::TomlAsset;
use crate::GameState;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Reflect)]
#[serde(tag = "type", content = "args")]
pub enum PopulationGenInfo {
    #[default]
    Bool,
    Int { range: (i32, i32) },
    Perm,
    Real { range: (f64, f64) },
}

#[derive(Default, Debug, Serialize, Deserialize, Reflect)]
pub struct PopulationConfig {
    pub dim: usize,
    pub pop_size: usize,
    pub cod: PopulationGenInfo,
}

#[derive(Default, Debug, Deserialize, Resource, Reflect)]
#[reflect(Resource)]
pub struct Config {
    pub population: PopulationConfig,
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Config>()
            .add_collection_to_loading_state::<_, ConfigAssets>(GameState::Loading)
            .add_systems(OnExit(GameState::Loading), parse_config);
    }
}

fn parse_config(
    mut commands: Commands,
    config_asset_handles: Res<ConfigAssets>,
    toml_assets: Res<Assets<TomlAsset>>,
) {
    let handle = &config_asset_handles.config;
    let content = toml_assets.get(handle).expect("Config Toml not loaded!");
    let config: Config = toml::from_str(&content.0).unwrap();
    commands.insert_resource(config);
}

#[derive(Resource, AssetCollection)]
pub struct ConfigAssets {
    #[asset(path = "config.toml")]
    pub config: Handle<TomlAsset>,
}
