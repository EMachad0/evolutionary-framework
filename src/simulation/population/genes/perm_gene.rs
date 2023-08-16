use bevy::prelude::Reflect;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::simulation::population::genes::Gene;

#[derive(Debug, Clone, Reflect)]
pub struct Perm(Vec<i32>);

impl Gene for Perm {
    type I = i32;
    type V = Vec<i32>;

    fn new(dim: &Self::I) -> Self {
        let mut identity = (0..*dim).collect::<Vec<i32>>();
        let mut rng = thread_rng();
        identity.shuffle(&mut rng);
        Self(identity)
    }

    fn get(&self) -> Self::V {
        self.0.clone()
    }
}
