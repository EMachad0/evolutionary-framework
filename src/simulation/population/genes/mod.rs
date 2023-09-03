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

pub trait GeneCod: 'static + Sync + Send + Clone {
    type I: Copy + Clone + Send + Sync;
    type G;

    fn new(dim: usize, input: &Self::I) -> Self;
    fn get(&self) -> &Vec<Self::G>;
    fn get_mut(&mut self) -> &mut Vec<Self::G>;
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Gene<G: GeneCod>(G);

impl<G: GeneCod> Gene<G> {
    pub fn get(&self) -> &Vec<G::G> {
        self.0.get()
    }

    pub fn get_mut(&mut self) -> &mut Vec<G::G> {
        self.0.get_mut()
    }
}

impl<G: GeneCod> From<G> for Gene<G> {
    fn from(value: G) -> Self {
        Self(value)
    }
}
