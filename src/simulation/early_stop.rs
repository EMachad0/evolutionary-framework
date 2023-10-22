use bevy::prelude::*;
use itertools::Itertools;

use crate::simulation::population::fitness::Fitness;
use crate::simulation::simulation_state::end_simulation;
use crate::simulation::{generation_counter, SimulationSchedule, SimulationSet};

pub struct EarlyStopPlugin;

impl Plugin for EarlyStopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            SimulationSchedule,
            (
                end_simulation
                    .run_if(is_optimized)
                    .after(SimulationSet::Fitness),
                end_simulation
                    .run_if(generation_counter::generation_counter_just_finished)
                    .after(generation_counter::update_counter),
            ),
        );
    }
}

pub fn is_converged(query: Query<&Fitness>) -> bool {
    query.iter().map(|f| f.get()).all_equal()
}

pub fn is_optimized(query: Query<&Fitness>) -> bool {
    query.iter().any(|f| f.get() == 1.)
}
