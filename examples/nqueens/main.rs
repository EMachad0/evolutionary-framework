// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod board_position;
mod fitness;
mod loading;
mod objective;
mod queen;

use bevy::prelude::*;
use bevy::DefaultPlugins;

use crate::board_position::BoardPosition;
use evolutionary_framework::despawn::despawn;
use evolutionary_framework::simulation::selected_individuals::select_best_individual;
use evolutionary_framework::simulation::simulation_state::is_simulation_paused;
use evolutionary_framework::simulation::{SimulationSchedule, SimulationSet};
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
                resolution: (1366., 1024.5).into(),
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
            loading::LoadingPlugin,
            objective::ObjectivePlugin,
            fitness::FitnessPlugin,
        ))
        .register_type::<board_position::BoardPosition>()
        .add_systems(Startup, set_window_icon)
        .add_systems(
            OnEnter(GameState::Playing),
            ((
                board::spawn_board,
                (board::spawn_board_cells, queen::spawn_queens),
            )
                .chain(),),
        )
        .add_systems(
            PreUpdate,
            (
                board_position::transform_from_board_position,
                board::update_board_if_resize,
                queen::queens_from_selected_individual,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            SimulationSchedule,
            select_best_individual
                .run_if(not(is_simulation_paused))
                .after(SimulationSet::Fitness),
        )
        .add_systems(OnExit(GameState::Playing), despawn::<BoardPosition>)
        .run();
}
