mod bool_gene;
mod int_gene;
mod perm_gene;
mod real_gene;

use bevy::prelude::*;
use std::fmt::Debug;

pub use bool_gene::Bool;
pub use int_gene::Int;
pub use perm_gene::Perm;
pub use real_gene::Real;

pub trait GeneCod {
    type I: Debug + Default + Copy + Clone + Send + Sync;
    type G;

    fn new(dim: usize, input: &Self::I) -> Self;
    fn get(&self) -> &Self::G;
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Gene<G: GeneCod>(G);

impl<G: GeneCod> Gene<G> {
    pub fn get(&self) -> &G {
        &self.0
    }
}

impl<G: GeneCod> From<G> for Gene<G> {
    fn from(value: G) -> Self {
        Self(value)
    }
}
