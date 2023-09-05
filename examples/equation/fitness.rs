use bevy::prelude::*;

use crate::function::Function;
use crate::objective::EquationObjective;
use evolutionary_framework::simulation::population::fitness::Fitness;
use evolutionary_framework::simulation::{SimulationSchedule, SimulationSet};

pub struct FitnessPlugin;

impl Plugin for FitnessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            SimulationSchedule,
            calc_fitness
                .in_set(SimulationSet::Fitness)
                .run_if(any_with_component::<Function>()),
        );
    }
}

pub fn calc_fitness(
    function: Query<&Function>,
    mut individuals: Query<(&EquationObjective, &mut Fitness)>,
) {
    let function = function.single();
    let y_domain = function.y_domain;
    for (objective, mut fitness) in individuals.iter_mut() {
        let fit = (objective.y - y_domain.0) / (y_domain.1 - y_domain.0);
        // for min
        // let fit = 1. - fit;

        fitness.set(fit as f64);
    }
}
