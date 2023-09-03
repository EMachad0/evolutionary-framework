use bevy::prelude::*;
use itertools::Itertools;
use rand::distributions::Uniform;
use rand::prelude::*;

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
    let mut rng = thread_rng();
    let cut_rng = Uniform::new(1, config.population.dim - 1);
    let mut population = query.iter_mut().collect_vec();
    for gs in population.chunks_exact_mut(2) {
        if rng.gen_bool(config.selection.crossover_prob) {
            let [a, b]: &mut [_; 2] = gs.try_into().unwrap();

            let a_gene = a.get_mut();
            let b_gene = b.get_mut();

            let cut = cut_rng.sample(&mut rng);

            let a_tail = a_gene.split_off(cut);
            let x = b_gene.splice(cut.., a_tail);
            a_gene.extend(x);
        }
    }
}
