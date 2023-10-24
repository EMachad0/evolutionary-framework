use bevy::prelude::Reflect;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use crate::simulation::population::genes::Chromosome;

#[derive(Debug, Clone, Reflect)]
pub struct Real(Vec<f64>);

impl Chromosome for Real {
    type I = (f64, f64);
    type G = f64;

    fn new(dim: usize, (min, max): &Self::I) -> Self {
        let mut rng = thread_rng();
        let range = Uniform::new(min, max);
        let gene = (0..dim).map(|_| rng.sample(range)).collect::<Vec<f64>>();
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

    fn mutate(&mut self, prob: f64) {
        let genes = self.get_mut();
        let mut rng = thread_rng();
        for p in genes.iter_mut() {
            if rng.gen_bool(prob) {
                *p = rng.gen_range(0.0..=1.0);
            }
        }
    }
}
