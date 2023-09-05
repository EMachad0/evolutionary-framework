use bevy::prelude::*;

use crate::objective::RadioObjective;
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

pub fn calc_fitness(mut individuals: Query<(&RadioObjective, &mut Fitness)>) {
    for (objective, mut fitness) in individuals.iter_mut() {
        let fit = objective.gain() as f64 / 1360.;
        let hn = (objective.restriction() as f64 / 16.).max(0.);

        fitness.set(fit - hn);
    }
}
