pub mod genes;

use bevy::prelude::*;
use genes::Gene;

use crate::config::{Config, PopulationGenInfo};
use genes::{Bool, Int, Perm, Real};

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

pub fn create_population<G: Gene>(size: usize, dim: usize, i: G::I) -> Vec<Individual<G>> {
    let mut population = Vec::with_capacity(size);
    for _ in 0..size {
        let individual: Individual<G> = Individual::new(dim, &i);
        population.push(individual);
    }
    population
}

pub fn spawn_population(mut commands: Commands, config: Res<Config>) {
    let pop_size = config.population.pop_size;
    let pop_dim = config.population.dim;
    let names = (0..pop_size).map(|i| Name::new(format!("Individual {}", i)));
    match config.population.cod {
        PopulationGenInfo::Bool => {
            let population = create_population::<Bool>(pop_size, pop_dim, ());
            let entities = population.into_iter().zip(names);
            commands.spawn_batch(entities);
        }
        PopulationGenInfo::Int { range } => {
            let population = create_population::<Int>(pop_size, pop_dim, range);
            let entities = population.into_iter().zip(names);
            commands.spawn_batch(entities);
        }
        PopulationGenInfo::Perm => {
            let population = create_population::<Perm>(pop_size, 1, pop_dim as i32);
            let entities = population.into_iter().zip(names);
            commands.spawn_batch(entities);
        }
        PopulationGenInfo::Real { range } => {
            let population = create_population::<Real>(pop_size, pop_dim, range);
            let entities = population.into_iter().zip(names);
            commands.spawn_batch(entities);
        }
    }
}
