#![allow(clippy::type_complexity)]

mod auto_runner;
pub mod camera;
pub mod config;
pub mod loading;
pub mod run_counter;
pub mod simulation;
pub mod toml_asset;
pub mod ui;
pub mod window;
pub mod despawn;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}

pub struct EvolutionaryFrameworkPlugin;

impl Plugin for EvolutionaryFrameworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            camera::CameraPlugin,
            toml_asset::TomlAssetPlugin,
            loading::LoadingPlugin,
            simulation::SimulationPlugin,
            ui::UiPlugin::default(),
            run_counter::RunCounterPlugin,
            auto_runner::AutoRunnerPlugin,
            config::ConfigPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                // LogDiagnosticsPlugin::filtered(vec![FrameTimeDiagnosticsPlugin::FPS]),
                LogDiagnosticsPlugin::filtered(vec![]),
            ));
        }
    }
}
