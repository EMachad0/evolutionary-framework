pub mod population;

use bevy::prelude::*;

use population::genes::{Bool, Int, Perm, Real};
use population::init_params::PopulationInitParams;

use crate::simulation::population::fitness::Fitness;
use crate::simulation::population::genes::Gene;
use crate::simulation::population::individual::Individual;
use crate::GameState;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Gene<Bool>>()
            .register_type::<Gene<Int>>()
            .register_type::<Gene<Perm>>()
            .register_type::<Gene<Real>>()
            .register_type::<Individual>()
            .register_type::<Fitness>()
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    population::init_params::insert_population_init_params,
                    (
                        population::spawn_population::<Bool>
                            .run_if(resource_exists::<PopulationInitParams<Bool>>()),
                        population::spawn_population::<Int>
                            .run_if(resource_exists::<PopulationInitParams<Int>>()),
                        population::spawn_population::<Real>
                            .run_if(resource_exists::<PopulationInitParams<Real>>()),
                        population::spawn_population::<Perm>
                            .run_if(resource_exists::<PopulationInitParams<Perm>>()),
                    ),
                )
                    .chain(),
            );
    }
}
