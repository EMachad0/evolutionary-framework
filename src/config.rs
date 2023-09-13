use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simulation::fixed_timestep::DEFAULT_STEPS_PER_SECOND;
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Reflect)]
#[serde(default)]
pub struct SimulationConfig {
    pub target_generation: u64,
    pub steps_per_second: f64,
    pub runs: u64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            target_generation: 0,
            steps_per_second: DEFAULT_STEPS_PER_SECOND,
            runs: 0,
        }
    }
}

#[derive(Default, Debug, Copy, Clone, Deserialize, Resource, Reflect)]
#[reflect(Resource)]
pub struct Config {
    pub population: PopulationConfig,
    pub selection: SelectionConfig,
    #[serde(default)]
    pub simulation: SimulationConfig,
}

pub struct ConfigPlugin;

pub const CONFIG_SCHEDULE: OnExit<GameState> = OnExit(GameState::Loading);

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, SystemSet)]
pub struct ConfigSet;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Config>()
            .configure_set(CONFIG_SCHEDULE, ConfigSet);
    }
}

pub trait ConfigToml: Resource {
    fn handle(&self) -> &Handle<TomlAsset>;
}

pub fn parse_config<T: ConfigToml>(world: &mut World) {
    let config_toml: &T = world.resource::<T>();
    let toml_assets = world.resource::<Assets<TomlAsset>>();
    let handle = config_toml.handle();
    let content = toml_assets.get(handle).expect("Config Toml not loaded!");
    let config: Config = toml::from_str(&content.0).unwrap();
    world.insert_resource(config);
}
