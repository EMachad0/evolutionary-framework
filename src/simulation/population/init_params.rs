use bevy::prelude::*;

use crate::config::{Config, PopulationConfig, PopulationGenInfo};
use crate::simulation::population::genes::{Bool, Chromosome, Int, Perm, Real};

#[derive(Default, Debug, Copy, Clone, Resource)]
pub struct PopulationInitParams<G: Chromosome> {
    pub size: usize,
    pub dim: usize,
    pub arg: G::I,
}

impl<G: Chromosome> PopulationInitParams<G> {
    pub fn new(size: usize, dim: usize, arg: G::I) -> Self {
        Self { size, dim, arg }
    }
}

pub fn insert_population_init_params(world: &mut World) {
    let config = world.get_resource::<Config>().unwrap();
    let PopulationConfig { size, dim, cod } = config.population;
    match cod {
        PopulationGenInfo::Bool => {
            world.insert_resource(PopulationInitParams::<Bool>::new(size, dim, ()));
        }
        PopulationGenInfo::Int { range } => {
            world.insert_resource(PopulationInitParams::<Int>::new(size, dim, range));
        }
        PopulationGenInfo::Perm => {
            world.insert_resource(PopulationInitParams::<Perm>::new(size, dim, ()));
        }
        PopulationGenInfo::Real { range } => {
            world.insert_resource(PopulationInitParams::<Real>::new(size, dim, range));
        }
    };
}
