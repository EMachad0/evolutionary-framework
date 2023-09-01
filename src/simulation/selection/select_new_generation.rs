use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::config::Config;
use crate::simulation::population::fitness::Fitness;
use crate::simulation::population::genes::{Gene, GeneCod};
use crate::simulation::selection::elitism::Elitism;

pub fn select_new_generation<G>(
    config: Res<Config>,
    mut query: Query<(&mut Gene<G>, Option<&Elitism>, &Fitness)>,
) where
    G: 'static + Clone + Send + Sync + GeneCod,
{
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
