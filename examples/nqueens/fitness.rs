use bevy::prelude::*;

use crate::objective::NQueensObjective;
use evolutionary_framework::config::Config;
use evolutionary_framework::simulation::population::fitness::Fitness;
use evolutionary_framework::simulation::{SimulationSchedule, SimulationSet};

pub struct FitnessPlugin;

impl Plugin for FitnessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            SimulationSchedule,
            calc_fitness.in_set(SimulationSet::Fitness),
        );
    }
}

pub fn calc_fitness(
    mut individuals: Query<(&NQueensObjective, &mut Fitness)>,
    config: Res<Config>,
) {
    let n = config.population.dim as i32;
    let worst_case = n * (n - 1) / 2;
    for (objective, mut fitness) in individuals.iter_mut() {
        fitness.set((worst_case - objective.collisions) as f64 / worst_case as f64);
    }
}
