use bevy::prelude::Reflect;
use rand::{thread_rng, Rng};

use crate::simulation::population::genes::Gene;

#[derive(Debug, Copy, Clone, Reflect)]
pub struct Int(i32);

impl Gene for Int {
    type I = (i32, i32);
    type V = i32;

    fn new((min, max): &Self::I) -> Self {
        Self(thread_rng().gen_range(*min..=*max))
    }

    fn get(&self) -> Self::V {
        self.0
    }
}
