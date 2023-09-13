use crate::GameState;
use bevy::prelude::*;
use itertools::Itertools;

use crate::simulation::population::fitness::Fitness;
use crate::simulation::{SimulationSchedule, SimulationSet};

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct FitnessHistory {
    pub avg: Vec<f64>,
    pub best: Vec<f64>,
}

pub struct FitnessDiagnosticsPlugin;

impl Plugin for FitnessDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FitnessHistory>()
            .add_systems(OnEnter(GameState::Playing), init_history_diagnostics)
            .add_systems(
                SimulationSchedule,
                collect_diagnostics.after(SimulationSet::Fitness),
            );
    }
}

fn collect_diagnostics(mut history: ResMut<FitnessHistory>, query: Query<&Fitness>) {
    let genes = query.iter().map(|f| f.get()).collect_vec();
    history.best.push(
        *genes
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap(),
    );
    history
        .avg
        .push(genes.iter().sum::<f64>() / genes.len() as f64);
}

fn init_history_diagnostics(mut history: ResMut<FitnessHistory>) {
    history.avg.clear();
    history.best.clear();
}
