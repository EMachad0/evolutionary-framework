use bevy::prelude::*;

use crate::maze::Maze;
use crate::objective::MazeObjective;
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

pub fn calc_fitness(maze: Res<Maze>, mut individuals: Query<(&MazeObjective, &mut Fitness)>) {
    let worst_case = maze.width + maze.height;
    individuals
        .par_iter_mut()
        .for_each_mut(|(objective, mut fitness)| {
            let fit = (worst_case - objective.distance) as f64 / worst_case as f64;
            fitness.set(fit);
        });
}
