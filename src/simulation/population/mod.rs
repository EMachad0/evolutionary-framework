pub mod fitness;
pub mod genes;
pub mod individual;
pub mod init_params;
pub mod objective;
pub mod run_condition;
pub mod spawn_population;

use bevy::prelude::*;

use crate::despawn::despawn;
use crate::simulation::population::fitness::Fitness;
use crate::simulation::population::genes::{Bool, Gene, Int, Perm, Real};
use crate::simulation::population::individual::Individual;
use crate::simulation::population::init_params::PopulationInitParams;
use crate::simulation::SimulationSet;
use crate::GameState;

pub struct PopulationPlugin;

impl Plugin for PopulationPlugin {
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
                    init_params::insert_population_init_params,
                    (
                        spawn_population::spawn_population::<Bool>
                            .run_if(resource_exists::<PopulationInitParams<Bool>>()),
                        spawn_population::spawn_population::<Int>
                            .run_if(resource_exists::<PopulationInitParams<Int>>()),
                        spawn_population::spawn_population::<Real>
                            .run_if(resource_exists::<PopulationInitParams<Real>>()),
                        spawn_population::spawn_population::<Perm>
                            .run_if(resource_exists::<PopulationInitParams<Perm>>()),
                    ),
                )
                    .chain()
                    .in_set(SimulationSet::PopulationStart),
            )
            .add_systems(OnExit(GameState::Playing), despawn::<Individual>);
    }
}
