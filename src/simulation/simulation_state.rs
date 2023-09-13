use bevy::prelude::*;
use std::fmt;
use std::fmt::Formatter;

use crate::GameState;

pub struct SimulationStatePlugin;

impl Plugin for SimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_systems(OnEnter(GameState::Menu), run_simulation);
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
    Finished,
}

impl fmt::Display for SimulationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SimulationState::Running => f.write_str("Running"),
            SimulationState::Paused => f.write_str("Paused"),
            SimulationState::Finished => f.write_str("Finished"),
        }
    }
}

pub fn is_simulation_paused(state: Res<State<SimulationState>>) -> bool {
    *state.get() != SimulationState::Running
}

pub fn is_simulation_finished(state: Res<State<SimulationState>>) -> bool {
    *state.get() == SimulationState::Finished
}

pub fn run_simulation(mut state: ResMut<NextState<SimulationState>>) {
    state.set(SimulationState::Running);
}

pub fn pause_simulation(mut state: ResMut<NextState<SimulationState>>) {
    state.set(SimulationState::Paused);
}

pub fn end_simulation(mut state: ResMut<NextState<SimulationState>>) {
    state.set(SimulationState::Finished);
}

pub fn toggle_pause_simulation(
    state: Res<State<SimulationState>>,
    mut state_controller: ResMut<NextState<SimulationState>>,
) {
    let next_state = match state.get() {
        SimulationState::Running => SimulationState::Paused,
        SimulationState::Paused => SimulationState::Running,
        SimulationState::Finished => SimulationState::Finished,
    };
    state_controller.set(next_state);
}
