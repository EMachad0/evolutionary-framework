use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

use crate::config::Config;
use crate::simulation::evolutionary_steps::elitism::Elitism;
use crate::simulation::population::fitness::Fitness;
use crate::simulation::population::genes::{Bool, Chromosome, Gene, Int, Perm, Real};
use crate::simulation::population::run_condition::population_type;
use crate::simulation::{SimulationSchedule, SimulationSet};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            SimulationSchedule,
            (
                select::<Bool>.run_if(population_type::<Bool>),
                select::<Int>.run_if(population_type::<Int>),
                select::<Perm>.run_if(population_type::<Perm>),
                select::<Real>.run_if(population_type::<Real>),
            )
                .in_set(SimulationSet::Selection),
        );
    }
}

pub fn select<G: Chromosome>(
    config: Res<Config>,
    mut query: Query<(&mut Gene<G>, Option<&Elitism>, &Fitness)>,
) {
    let mut rng = thread_rng();

    let population = query.iter_mut().collect_vec();

    let select_amount = config.population.size - config.selection.elitism;
    let mut new_population: Vec<Gene<G>> = Vec::with_capacity(select_amount);
    for _ in 0..select_amount {
        let selected = population
            .choose_weighted(&mut rng, |(_, _, f)| f.get())
            .unwrap();
        let selected = selected.0.clone();
        new_population.push(selected)
    }

    let to_be_replaced = population
        .into_iter()
        .filter(|(_, e, _)| e.is_none())
        .map(|(g, _, _)| g)
        .collect_vec();

    to_be_replaced
        .into_iter()
        .zip(new_population.into_iter())
        .for_each(|(mut g, ng)| *g = ng);
}
