use bevy::prelude::*;
use rand::seq::IteratorRandom;
use rand::thread_rng;

use crate::simulation::population::individual::Individual;

pub struct SelectedIndividualsPlugin;

impl Plugin for SelectedIndividualsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedIndividuals>();
    }
}

#[derive(Default, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct SelectedIndividuals(Vec<Entity>);

impl SelectedIndividuals {
    pub fn get(&self) -> &Vec<Entity> {
        &self.0
    }

    pub fn single(&self) -> Option<Entity> {
        self.0.first().copied()
    }

    pub fn get_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.0
    }
}

pub fn select_random_individual(
    mut selection: ResMut<SelectedIndividuals>,
    query: Query<Entity, With<Individual>>,
) {
    let selected = selection.get_mut();
    loop {
        let entity = query.iter().choose(&mut thread_rng()).unwrap();
        if !selected.contains(&entity) {
            selected.push(entity);
            break;
        }
    }
}
