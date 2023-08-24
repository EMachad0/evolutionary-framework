use bevy::prelude::Reflect;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::simulation::population::genes::GeneCod;

#[derive(Debug, Clone, Reflect)]
pub struct Perm(Vec<i32>);

impl GeneCod for Perm {
    type I = ();
    type G = Vec<i32>;

    fn new(dim: usize, _: &Self::I) -> Self {
        let mut gene = (0..dim as i32).collect::<Vec<i32>>();
        let mut rng = thread_rng();
        gene.shuffle(&mut rng);
        Self(gene)
    }

    fn get(&self) -> &Self::G {
        &self.0
    }
}
