mod population;

use bevy::prelude::*;

use population::genes::{Bool, Int, Perm, Real};

use crate::simulation::population::Individual;
use crate::GameState;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Individual<Bool>>()
            .register_type::<Individual<Int>>()
            .register_type::<Individual<Perm>>()
            .register_type::<Individual<Real>>()
            .add_systems(OnEnter(GameState::Playing), population::spawn_population);
    }
}
