use bevy::prelude::*;

#[derive(Default, Debug, Copy, Clone, Component, Reflect)]
pub struct Fitness(f64);

impl Fitness {
    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn set(&mut self, value: f64) {
        self.0 = value;
    }
}
