// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod camera;
mod cartesian_plane;
mod config;
mod fitness;
mod function;
mod individual;

use bevy::prelude::*;
use bevy::DefaultPlugins;

use evolutionary_framework::window::set_window_icon;
use evolutionary_framework::EvolutionaryFrameworkPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Evolutionary Framework Equation".to_string(),
                resolution: (800., 600.).into(),
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            EvolutionaryFrameworkPlugin,
            camera::CameraPlugin,
            cartesian_plane::CartesianPlanePlugin,
            function::FunctionPlugin,
            individual::IndividualPlugin,
            fitness::FitnessPlugin,
            config::ConfigPlugin,
        ))
        .add_systems(Startup, set_window_icon)
        .run();
}
