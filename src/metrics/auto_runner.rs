use bevy::prelude::*;

use crate::metrics::run_counter::run_counter_finished;
use crate::simulation::simulation_state::is_simulation_finished;
use crate::GameState;

pub struct AutoRunnerPlugin;

impl Plugin for AutoRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            start_simulation.run_if(not(run_counter_finished)),
        )
        .add_systems(
            PostUpdate,
            end_simulation.run_if(
                in_state(GameState::Playing)
                    .and_then(is_simulation_finished)
                    .and_then(not(run_counter_finished)),
            ),
        );
    }
}

pub fn start_simulation(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}

pub fn end_simulation(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Menu);
}
