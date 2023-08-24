#![allow(clippy::type_complexity)]

pub mod camera;
pub mod config;
pub mod loading;
pub mod selected_individual;
pub mod simulation;
pub mod toml_asset;
pub mod ui;
pub mod window;

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
            config::ConfigPlugin,
            loading::LoadingPlugin,
            simulation::SimulationPlugin,
            toml_asset::TomlAssetPlugin,
            ui::UiPlugin::default(),
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::filtered(vec![FrameTimeDiagnosticsPlugin::FPS]),
            ));
        }
    }
}
