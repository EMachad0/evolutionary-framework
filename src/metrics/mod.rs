use bevy::prelude::*;

use crate::config::Config;

pub mod auto_runner;
pub mod fitness_csv;
pub mod run_counter;

pub struct MetricsPlugin;

impl Plugin for MetricsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            auto_runner::AutoRunnerPlugin,
            run_counter::RunCounterPlugin,
            fitness_csv::FitnessCsvPlugin,
        ));
    }
}

pub fn is_auto_runner(config: Res<Config>) -> bool {
    config.simulation.runs > 0
}
