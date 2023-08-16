use bevy::prelude::Reflect;
use rand::random;

use crate::simulation::population::genes::Gene;

#[derive(Debug, Copy, Clone, Reflect)]
pub struct Bool(bool);

impl Gene for Bool {
    type I = ();
    type V = bool;

    fn new((): &Self::I) -> Self {
        Self(random::<bool>())
    }

    fn get(&self) -> Self::V {
        self.0
    }
}
