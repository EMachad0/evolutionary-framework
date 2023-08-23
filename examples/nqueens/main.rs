// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod board_position;

use bevy::prelude::*;
use bevy::DefaultPlugins;

use evolutionary_framework::window::set_window_icon;
use evolutionary_framework::{EvolutionaryFrameworkPlugin, GameState};

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(
            Color::hex("80CEE1").expect("Unable to parse clear color hex"),
        ))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Evolutionary Framework NQueens".to_string(),
                resolution: (800., 600.).into(),
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EvolutionaryFrameworkPlugin)
        .register_type::<board_position::BoardPosition>()
        .add_systems(Startup, set_window_icon)
        .add_systems(OnEnter(GameState::Playing), board::spawn_board)
        .add_systems(
            PreUpdate,
            (
                board_position::transform_from_board_position,
                board::update_board_if_window_resize,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}
