mod population;

use bevy::prelude::*;

use population::genes::{Bool, Int, Perm, Real};
use population::init_params::PopulationInitParams;

use crate::simulation::population::Individual;
use crate::GameState;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Individual<Bool>>()
            .register_type::<Individual<Int>>()
            .register_type::<Individual<Perm>>()
            .register_type::<Individual<Real>>()
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
