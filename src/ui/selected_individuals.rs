use bevy::prelude::*;
use bevy::utils::HashSet;
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
pub struct SelectedIndividuals(HashSet<Entity>);

impl SelectedIndividuals {
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }

    pub fn single(&self) -> Option<&Entity> {
        self.0.iter().next()
    }

    pub fn contains(&self, entity: &Entity) -> bool {
        self.0.contains(entity)
    }

    pub fn select(&mut self, entity: Entity) {
        if !self.0.remove(&entity) {
            self.0.insert(entity);
        }
    }

    pub fn replace(&mut self, entity: Entity) {
        self.0.clear();
        self.0.insert(entity);
    }
}

pub fn select_random_individual(
    mut selection: ResMut<SelectedIndividuals>,
    query: Query<Entity, With<Individual>>,
) {
    loop {
        let entity = query.iter().choose(&mut thread_rng()).unwrap();
        if !selection.contains(&entity) {
            selection.select(entity);
            break;
        }
    }
}
