use bevy::prelude::*;

use crate::board_weight::BoardWeight;
use crate::objective::NQueensObjective;
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
    weights: Res<BoardWeight>,
) {
    let n = weights.board_size;
    let max_collisions = n * (n - 1) / 2;
    let max_score = weights.max_score;
    for (objective, mut fitness) in individuals.iter_mut() {
        let fit = objective.score / max_score;
        let penalty = objective.collisions as f64 / max_collisions as f64;

        fitness.set((fit - penalty).max(0.));
    }
}
