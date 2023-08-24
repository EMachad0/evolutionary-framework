pub mod fitness;
pub mod genes;
pub mod individual;
pub mod init_params;

use crate::simulation::population::fitness::Fitness;
use bevy::prelude::*;

use crate::simulation::population::genes::Gene;
use crate::simulation::population::genes::GeneCod;
use crate::simulation::population::individual::Individual;
use crate::simulation::population::init_params::PopulationInitParams;

pub fn spawn_population<G>(mut commands: Commands, population_cod: Res<PopulationInitParams<G>>)
where
    G: 'static + Send + Sync + GeneCod,
{
    let PopulationInitParams { size, dim, arg } = *population_cod;

    let population = (0..size)
        .map(|i| {
            let name = Name::new(format!("Individual {}", i));
            let gene = Gene::from(G::new(dim, &arg));
            (name, gene, Fitness::default(), Individual)
        })
        .collect::<Vec<_>>();

    commands.spawn_batch(population);
}
