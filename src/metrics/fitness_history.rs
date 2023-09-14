use crate::metrics::is_auto_runner;
use crate::GameState;
use bevy::prelude::*;

use crate::simulation::fitness_diagnostics::FitnessDiagnostics;

pub struct RunsFitnessHistoryPlugin;

impl Plugin for RunsFitnessHistoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunsFitnessHistory>().add_systems(
            OnExit(GameState::Playing),
            update_fitness_history.run_if(is_auto_runner),
        );
    }
}

/// Fitness values over many runs
#[derive(Debug, Default, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct RunsFitnessHistory(Vec<FitnessDiagnostics>);

pub fn update_fitness_history(
    mut hist_fit: ResMut<RunsFitnessHistory>,
    run_fit: Res<FitnessDiagnostics>,
) {
    hist_fit.push(run_fit.clone());
}
