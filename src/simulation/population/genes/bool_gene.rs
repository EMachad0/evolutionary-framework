use bevy::prelude::Reflect;
use rand::distributions::Bernoulli;
use rand::prelude::*;

use crate::simulation::population::genes::Chromosome;

#[derive(Debug, Clone, Reflect)]
pub struct Bool(pub Vec<bool>);

impl Chromosome for Bool {
    type I = ();
    type G = bool;

    fn new(dim: usize, (): &Self::I) -> Self {
        let mut rng = thread_rng();
        let gene = (0..dim).map(|_| rng.gen()).collect::<Vec<_>>();
        Self(gene)
    }

    fn get(&self) -> &Vec<Self::G> {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Vec<Self::G> {
        &mut self.0
    }

    fn crossover(&mut self, other: &mut Self, prob: f64) {
        let mut rng = thread_rng();
        if rng.gen_bool(prob) {
            let cut = rng.gen_range(1..self.0.len());
            let a_gene = self.get_mut();
            let b_gene = other.get_mut();

            let a_tail = a_gene.split_off(cut);
            let x = b_gene.splice(cut.., a_tail);
            a_gene.extend(x);
        }
    }

    fn mutate(&mut self, prob: f64) {
        let mut rng = thread_rng();
        let distribution = Bernoulli::new(prob).unwrap();
        for bit in self.0.iter_mut() {
            if distribution.sample(&mut rng) {
                *bit = !*bit
            }
        }
    }
}
