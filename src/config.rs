use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::toml_asset::TomlAsset;
use crate::GameState;

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Reflect)]
#[serde(tag = "type", content = "args")]
pub enum PopulationGenInfo {
    #[default]
    Bool,
    Int {
        range: (i32, i32),
    },
    Perm,
    Real {
        range: (f64, f64),
    },
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, Reflect)]
pub struct PopulationConfig {
    pub dim: usize,
    pub size: usize,
    pub cod: PopulationGenInfo,
}

#[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, Reflect)]
pub struct SelectionConfig {
    pub elitism: usize,
    pub crossover_prob: f64,
    pub mutation_prob: f64,
}

#[derive(Default, Debug, Copy, Clone, Deserialize, Resource, Reflect)]
#[reflect(Resource)]
pub struct Config {
    pub population: PopulationConfig,
    pub selection: SelectionConfig,
}

pub struct ConfigPlugin;

pub const CONFIG_SCHEDULE: OnExit<GameState> = OnExit(GameState::Loading);

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Config>();
    }
}

pub trait ConfigToml {
    fn handle(&self) -> &Handle<TomlAsset>;
}

pub fn parse_config<T: ConfigToml + Resource>(
    mut commands: Commands,
    config_toml: Res<T>,
    toml_assets: Res<Assets<TomlAsset>>,
) {
    let handle = config_toml.handle();
    let content = toml_assets.get(handle).expect("Config Toml not loaded!");
    let config: Config = toml::from_str(&content.0).unwrap();
    commands.insert_resource(config);
}
