use bevy::prelude::Reflect;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::simulation::population::genes::Chromosome;

#[derive(Debug, Clone, Reflect)]
pub struct Perm(Vec<i32>);

impl Chromosome for Perm {
    type I = ();
    type G = i32;

    fn new(dim: usize, _: &Self::I) -> Self {
        let mut gene = (0..dim as i32).collect::<Vec<i32>>();
        let mut rng = thread_rng();
        gene.shuffle(&mut rng);
        Self(gene)
    }

    fn get(&self) -> &Vec<Self::G> {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Vec<Self::G> {
        &mut self.0
    }

    fn mutate(&mut self, _prob: f64) {
        todo!()
    }
}
