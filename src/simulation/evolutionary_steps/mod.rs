pub mod crossover;
pub mod elitism;
pub mod mutation;
pub mod selection;

use bevy::prelude::*;

pub struct EvolutionaryStepsPlugin;

impl Plugin for EvolutionaryStepsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            elitism::ElitismPlugin,
            selection::SelectionPlugin,
            crossover::CrossoverPlugin,
            mutation::MutationPlugin,
        ));
    }
}
