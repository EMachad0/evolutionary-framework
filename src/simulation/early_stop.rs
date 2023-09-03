use bevy::prelude::*;
use itertools::Itertools;

use crate::simulation::population::fitness::Fitness;
use crate::simulation::simulation_state::pause_simulation;

pub struct EarlyStopPlugin;

impl Plugin for EarlyStopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                pause_simulation.run_if(is_converged),
                pause_simulation.run_if(is_optimized),
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
