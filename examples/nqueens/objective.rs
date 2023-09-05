use bevy::prelude::*;
use std::fmt;

use evolutionary_framework::simulation::population::genes::{Gene, Perm};
use evolutionary_framework::simulation::population::objective::{
    objective_setup, update_name_from_objective, Objective,
};
use evolutionary_framework::simulation::{SimulationSchedule, SimulationSet};
use evolutionary_framework::GameState;

pub struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            objective_setup::<NQueensObjective>.after(SimulationSet::PopulationStart),
        )
        .add_systems(
            SimulationSchedule,
            calc_objective.in_set(SimulationSet::Objective),
        )
        .add_systems(PostUpdate, update_name_from_objective::<NQueensObjective>);
    }
}

#[derive(Debug, Default, Copy, Clone, Component, Reflect)]
pub struct NQueensObjective {
    pub collisions: i32,
}

impl Objective for NQueensObjective {}

impl fmt::Display for NQueensObjective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("collisions={}", self.collisions))
    }
}

pub fn calc_objective(mut query: Query<(&mut NQueensObjective, &Gene<Perm>)>) {
    for (mut objective, gene) in query.iter_mut() {
        let perm = gene.get();
        let n = perm.len();
        let mut diagonals1: Vec<i32> = vec![0; n * 2];
        let mut diagonals2: Vec<i32> = vec![0; n * 2];
        let mut collisions = 0;

        for (x, y) in perm.iter().enumerate() {
            let y = *y as usize;
            collisions += diagonals1[n + x - y];
            collisions += diagonals2[x + y];
            diagonals1[n + x - y] += 1;
            diagonals2[x + y] += 1;
        }

        objective.collisions = collisions;
    }
}
