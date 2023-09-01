use bevy::prelude::*;
use itertools::Itertools;

use crate::config::Config;
use crate::simulation::population::fitness::Fitness;

#[derive(Debug, Component, Reflect)]
pub struct Elitism;

pub fn is_elitist(config: Res<Config>) -> bool {
    config.selection.elitism > 0
}

pub fn select_elitism(
    mut commands: Commands,
    config: Res<Config>,
    query: Query<(Entity, &Fitness)>,
) {
    query
        .iter()
        .sorted_by(|(_, a), (_, b)| a.get().partial_cmp(&b.get()).unwrap().reverse())
        .take(config.selection.elitism)
        .map(|(entity, _)| entity)
        .for_each(|entity| {
            commands.entity(entity).insert(Elitism);
        });
}

pub fn clean_elitism(mut commands: Commands, query: Query<Entity, With<Elitism>>) {
    query.for_each(|entity| {
        commands.entity(entity).remove::<Elitism>();
    })
}
