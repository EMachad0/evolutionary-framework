use bevy::prelude::*;
use itertools::Itertools;

use crate::config::Config;
use crate::simulation::population::fitness::Fitness;
use crate::simulation::{SimulationSchedule, SimulationSet};

pub struct ElitismPlugin;

impl Plugin for ElitismPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Elitism>().add_systems(
            SimulationSchedule,
            (clean_elitism, select_elitism)
                .run_if(is_elitism_enabled)
                .chain()
                .in_set(SimulationSet::Elitism),
        );
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Elitism;

pub fn is_elitism_enabled(config: Res<Config>) -> bool {
    config.selection.elitism > 0
}

pub fn select_elitism(world: &mut World) {
    let elitism = world.resource::<Config>().selection.elitism;
    let mut query = world
        .query::<(Entity, &Fitness)>()
        .iter_mut(world)
        .collect_vec();

    query.sort_by(|(_, a), (_, b)| a.get().partial_cmp(&b.get()).unwrap().reverse());

    let query = query
        .into_iter()
        .take(elitism)
        .map(|(entity, _)| entity)
        .collect_vec();

    query.into_iter().for_each(|entity| {
        world.entity_mut(entity).insert(Elitism);
    });
}

pub fn clean_elitism(world: &mut World) {
    let query = world
        .query_filtered::<Entity, With<Elitism>>()
        .iter_mut(world)
        .collect_vec();
    for entity in query {
        world.entity_mut(entity).remove::<Elitism>();
    }
}
