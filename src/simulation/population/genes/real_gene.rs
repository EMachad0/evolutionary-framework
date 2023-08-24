use bevy::prelude::Reflect;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use crate::simulation::population::genes::GeneCod;

#[derive(Debug, Clone, Reflect)]
pub struct Real(Vec<f64>);

impl GeneCod for Real {
    type I = (f64, f64);
    type G = Vec<f64>;

    fn new(dim: usize, (min, max): &Self::I) -> Self {
        let mut rng = thread_rng();
        let range = Uniform::new(min, max);
        let gene = (0..dim).map(|_| rng.sample(range)).collect::<Vec<f64>>();
        Self(gene)
    }

    fn get(&self) -> &Self::G {
        &self.0
    }
}
