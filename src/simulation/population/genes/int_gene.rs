use bevy::prelude::Reflect;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use crate::simulation::population::genes::GeneCod;

#[derive(Debug, Clone, Reflect)]
pub struct Int(Vec<i32>);

impl GeneCod for Int {
    type I = (i32, i32);
    type G = Vec<i32>;

    fn new(dim: usize, (min, max): &Self::I) -> Self {
        let mut rng = thread_rng();
        let range = Uniform::new(min, max);
        let gene = (0..dim).map(|_| rng.sample(range)).collect::<Vec<_>>();
        Self(gene)
    }

    fn get(&self) -> &Self::G {
        &self.0
    }
}
