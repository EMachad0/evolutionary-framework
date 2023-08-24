use bevy::prelude::*;

use crate::simulation::population::genes::Perm;
use crate::simulation::population::Individual;

#[derive(Default, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct SelectedIndividuals(Vec<Entity>);

impl SelectedIndividuals {
    pub fn get(&self) -> &Vec<Entity> {
        &self.0
    }

    pub fn single(&self) -> Entity {
        if self.0.len() != 1 {
            panic!(
                "expect to only have a single selected entity but found {}",
                self.0.len()
            );
        }
        self.0.first().unwrap().to_owned()
    }

    pub fn get_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.0
    }
}

pub fn select_random_individual() {
    
}
