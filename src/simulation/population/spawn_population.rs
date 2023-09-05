use bevy::prelude::*;

use crate::simulation::population::fitness::Fitness;
use crate::simulation::population::genes::{Chromosome, Gene};
use crate::simulation::population::individual::Individual;
use crate::simulation::population::init_params::PopulationInitParams;

pub fn spawn_population<G: Chromosome>(world: &mut World) {
    let population_cod = world.resource::<PopulationInitParams<G>>();
    let PopulationInitParams { size, dim, arg } = *population_cod;

    let population = (0..size)
        .map(|i| {
            let name = Name::new(format!("Individual {}", i));
            let gene = Gene::from(G::new(dim, &arg));
            (name, gene, Fitness::default(), Individual)
        })
        .collect::<Vec<_>>();

    world.spawn_batch(population);
}
