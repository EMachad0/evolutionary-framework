use bevy::prelude::*;
use itertools::Itertools;

use crate::metrics::is_auto_runner;
use crate::metrics::run_counter::RunCounter;
use crate::simulation::fitness_diagnostics::FitnessDiagnostics;
use crate::GameState;

pub struct FitnessCsvPlugin;

impl Plugin for FitnessCsvPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            generate_csv.run_if(is_auto_runner),
        );
    }
}

pub fn generate_csv(runs: Res<FitnessDiagnostics>, counter: Res<RunCounter>) {
    let run_idx = counter.current;
    let path = format!("out/fitness_run_{}.csv", run_idx);
    let output = std::fs::File::create(path).unwrap();
    let mut wtr = csv::Writer::from_writer(output);
    for vec in runs.iter() {
        let run = vec.iter().map(|f| f.to_string()).collect_vec();
        wtr.write_record(run).unwrap();
    }
    wtr.flush().unwrap();
}
