use bevy::prelude::*;

use crate::function::Function;
use evolutionary_framework::simulation::population::fitness::Fitness;
use evolutionary_framework::simulation::population::genes::{Bool, Gene};
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
    mut individuals: Query<(&Gene<Bool>, &mut Fitness)>,
) {
    let function = function.single();
    let x_domain = function.x_domain;
    let y_domain = function.y_domain;
    for (gene, mut fitness) in individuals.iter_mut() {
        let x = gene_to_value(gene) * (x_domain.1 - x_domain.0) + x_domain.0;
        let y = (function.f)(x);

        let fit = (y - y_domain.0) / (y_domain.1 - y_domain.0);
        // for min
        // let fit = 1. - fit;

        fitness.set(fit as f64);
    }
}

pub fn gene_to_value(gene: &Gene<Bool>) -> f32 {
    let gene = gene.get();
    let mut dec = 0.;
    for b in gene.iter().rev() {
        if *b {
            dec += 1.;
        }
        dec /= 2.;
    }
    dec
}
