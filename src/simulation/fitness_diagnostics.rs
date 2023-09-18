use crate::GameState;
use bevy::prelude::*;
use itertools::Itertools;

use crate::simulation::population::fitness::Fitness;
use crate::simulation::{SimulationSchedule, SimulationSet};

#[derive(Debug, Default, Clone, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct FitnessDiagnostics(Vec<Vec<f64>>);

pub struct FitnessDiagnosticsPlugin;

impl Plugin for FitnessDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FitnessDiagnostics>()
            .add_systems(OnEnter(GameState::Playing), init_history_diagnostics)
            .add_systems(
                SimulationSchedule,
                collect_diagnostics.after(SimulationSet::Fitness),
            );
    }
}

fn collect_diagnostics(mut history: ResMut<FitnessDiagnostics>, query: Query<&Fitness>) {
    let genes = query.iter().map(|f| f.get()).collect_vec();
    history.push(genes);
}

fn init_history_diagnostics(mut history: ResMut<FitnessDiagnostics>) {
    history.clear();
}
