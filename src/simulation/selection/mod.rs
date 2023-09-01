pub mod elitism;
pub mod select_new_generation;

use bevy::prelude::*;

use crate::simulation::population::genes::{Bool, Int, Perm, Real};
use crate::simulation::population::run_condition::population_type;
use crate::simulation::selection::elitism::is_elitist;
use crate::simulation::{SelectionSchedule, SimulationSet};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<elitism::Elitism>()
            .add_systems(
                SelectionSchedule,
                (elitism::clean_elitism, elitism::select_elitism)
                    .run_if(is_elitist)
                    .chain()
                    .in_set(SimulationSet::Elitism),
            )
            .add_systems(
                SelectionSchedule,
                (
                    select_new_generation::select_new_generation::<Bool>
                        .run_if(population_type::<Bool>),
                    select_new_generation::select_new_generation::<Int>
                        .run_if(population_type::<Int>),
                    select_new_generation::select_new_generation::<Perm>
                        .run_if(population_type::<Perm>),
                    select_new_generation::select_new_generation::<Real>
                        .run_if(population_type::<Real>),
                )
                    .in_set(SimulationSet::Selection),
            );
    }
}
