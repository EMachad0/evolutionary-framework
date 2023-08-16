use bevy::prelude::Reflect;
use rand::{thread_rng, Rng};

use crate::simulation::population::genes::Gene;

#[derive(Debug, Copy, Clone, Reflect)]
pub struct Real(f64);

impl Gene for Real {
    type I = (f64, f64);
    type V = f64;

    fn new((min, max): &Self::I) -> Self {
        Self(thread_rng().gen_range(*min..=*max))
    }

    fn get(&self) -> Self::V {
        self.0
    }
}
