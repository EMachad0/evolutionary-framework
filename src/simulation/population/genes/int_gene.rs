use bevy::prelude::Reflect;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use crate::simulation::population::genes::Chromosome;

#[derive(Debug, Clone, Reflect)]
pub struct Int(Vec<i32>);

impl Chromosome for Int {
    type I = (i32, i32);
    type G = i32;

    fn new(dim: usize, (min, max): &Self::I) -> Self {
        let mut rng = thread_rng();
        let range = Uniform::new(min, max);
        let gene = (0..dim).map(|_| rng.sample(range)).collect::<Vec<_>>();
        Self(gene)
    }

    fn get(&self) -> &Vec<Self::G> {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Vec<Self::G> {
        &mut self.0
    }

    fn crossover(&mut self, _other: &mut Self, _prob: f64) {
        todo!()
    }

    fn mutate(&mut self, _prob: f64) {
        todo!()
    }
}
