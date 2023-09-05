use bevy::prelude::*;

use crate::simulation::population::individual::Individual;

pub trait Objective: Default + Component {}

pub fn objective_setup<T: Objective>(
    mut commands: Commands,
    query: Query<Entity, With<Individual>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(T::default());
    }
}

pub fn update_name_from_objective<T: Objective + std::fmt::Display>(
    mut query: Query<(&mut Name, &T)>,
) {
    for (mut name, objective) in query.iter_mut() {
        name.set(objective.to_string())
    }
}
