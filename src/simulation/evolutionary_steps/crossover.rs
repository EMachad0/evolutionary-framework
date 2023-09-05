use bevy::prelude::*;
use itertools::Itertools;

use crate::config::Config;
use crate::simulation::evolutionary_steps::elitism::Elitism;
use crate::simulation::population::genes::{Bool, Chromosome, Gene, Int, Perm, Real};
use crate::simulation::population::run_condition::population_type;
use crate::simulation::{SimulationSchedule, SimulationSet};

pub struct CrossoverPlugin;

impl Plugin for CrossoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            SimulationSchedule,
            (
                crossover::<Bool>.run_if(population_type::<Bool>),
                crossover::<Int>.run_if(population_type::<Int>),
                crossover::<Perm>.run_if(population_type::<Perm>),
                crossover::<Real>.run_if(population_type::<Real>),
            )
                .run_if(is_crossover_enabled)
                .in_set(SimulationSet::Crossover),
        );
    }
}

pub fn is_crossover_enabled(config: Res<Config>) -> bool {
    config.population.dim >= 2 && config.selection.crossover_prob > 0.
}

pub fn crossover<G: Chromosome>(
    config: Res<Config>,
    mut query: Query<&mut Gene<G>, Without<Elitism>>,
) {
    let mut population = query.iter_mut().collect_vec();
    for gs in population.chunks_exact_mut(2) {
        let [a, b]: &mut [_; 2] = gs.try_into().unwrap();
        a.crossover(b, config.selection.crossover_prob);
    }
}
