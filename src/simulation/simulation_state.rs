use bevy::prelude::*;
use std::fmt;
use std::fmt::Formatter;

use crate::GameState;

pub struct SimulationStatePlugin;

impl Plugin for SimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationState>()
            .add_systems(OnEnter(GameState::Menu), run_simulation);
    }
}

#[derive(Debug, Default, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct SimulationState(SimulationStateType);

impl SimulationState {
    pub fn get(&self) -> &SimulationStateType {
        &self.0
    }

    pub fn set(&mut self, state: SimulationStateType) {
        self.0 = state;
    }
}

impl fmt::Display for SimulationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.get().to_string())
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SimulationStateType {
    #[default]
    Running,
    Paused,
    Finished,
}

impl fmt::Display for SimulationStateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SimulationStateType::Running => f.write_str("Running"),
            SimulationStateType::Paused => f.write_str("Paused"),
            SimulationStateType::Finished => f.write_str("Finished"),
        }
    }
}

pub fn is_simulation_paused(state: Res<SimulationState>) -> bool {
    *state.get() != SimulationStateType::Running
}

pub fn is_simulation_finished(state: Res<SimulationState>) -> bool {
    *state.get() == SimulationStateType::Finished
}

pub fn run_simulation(mut state: ResMut<SimulationState>) {
    state.set(SimulationStateType::Running);
}

pub fn pause_simulation(mut state: ResMut<SimulationState>) {
    state.set(SimulationStateType::Paused);
}

pub fn end_simulation(mut state: ResMut<SimulationState>) {
    state.set(SimulationStateType::Finished);
}

pub fn toggle_pause_simulation(mut state: ResMut<SimulationState>) {
    let next_state = match state.get() {
        SimulationStateType::Running => SimulationStateType::Paused,
        SimulationStateType::Paused => SimulationStateType::Running,
        SimulationStateType::Finished => SimulationStateType::Finished,
    };
    state.set(next_state);
}
