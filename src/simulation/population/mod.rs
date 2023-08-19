pub mod genes;
pub mod init_params;

use bevy::prelude::*;
use bevy::sprite::Material2d;
use genes::Gene;
use init_params::PopulationInitParams;

#[derive(Debug, Clone, Component, Reflect)]
pub struct Individual<G: Gene>(pub Vec<G>);

impl<G: Gene> Individual<G> {
    pub fn new(dim: usize, i: &G::I) -> Self {
        let mut genes = Vec::with_capacity(dim);
        for _ in 0..dim {
            genes.push(G::new(i));
        }
        Self(genes)
    }
}

pub fn spawn_population<G>(mut commands: Commands, population_cod: Res<PopulationInitParams<G>>)
where
    G: 'static + Send + Sync + Gene,
{
    let PopulationInitParams { size, dim, arg } = *population_cod;

    let population = (0..size)
        .map(|i| {
            let name = Name::new(format!("Individual {}", i));
            let individual = Individual::<G>::new(dim, &arg);
            (name, individual)
        })
        .collect::<Vec<_>>();

    commands.spawn_batch(population);
}
