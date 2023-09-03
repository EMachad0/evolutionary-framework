use bevy::prelude::Reflect;
use rand::Rng;

use crate::simulation::population::genes::GeneCod;

#[derive(Debug, Clone, Reflect)]
pub struct Bool(Vec<bool>);

impl GeneCod for Bool {
    type I = ();
    type G = bool;

    fn new(dim: usize, (): &Self::I) -> Self {
        let mut rng = rand::thread_rng();
        let gene = (0..dim).map(|_| rng.gen()).collect::<Vec<_>>();
        Self(gene)
    }

    fn get(&self) -> &Vec<Self::G> {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Vec<Self::G> {
        &mut self.0
    }
}
