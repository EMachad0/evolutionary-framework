use bevy::prelude::*;

use crate::config::Config;
use crate::simulation::evolutionary_steps::elitism::Elitism;
use crate::simulation::population::genes::{Bool, Chromosome, Gene, Int, Perm, Real};
use crate::simulation::population::run_condition::population_type;
use crate::simulation::{SimulationSchedule, SimulationSet};

pub struct MutationPlugin;

impl Plugin for MutationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            SimulationSchedule,
            (
                mutation::<Bool>.run_if(population_type::<Bool>),
                mutation::<Int>.run_if(population_type::<Int>),
                mutation::<Perm>.run_if(population_type::<Perm>),
                mutation::<Real>.run_if(population_type::<Real>),
            )
                .run_if(is_mutation_enabled)
                .in_set(SimulationSet::Mutation),
        );
    }
}

pub fn is_mutation_enabled(config: Res<Config>) -> bool {
    config.selection.mutation_prob > 0.
}

pub fn mutation<G: Chromosome>(
    config: Res<Config>,
    mut query: Query<&mut Gene<G>, Without<Elitism>>,
) {
    query.par_iter_mut().for_each_mut(|mut gene| {
        gene.mutate(config.selection.mutation_prob);
    });
}
