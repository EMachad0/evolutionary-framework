pub mod early_stop;
pub mod evolutionary_steps;
pub mod fitness_diagnostics;
pub mod fixed_timestep;
pub mod generation_counter;
pub mod population;
pub mod selected_individuals;
pub mod simulation_state;
pub mod simulation_timer;

use bevy::ecs::schedule::ScheduleLabel;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, SystemSet)]
pub enum SimulationSet {
    PopulationStart,
    Objective,
    Fitness,
    Elitism,
    Selection,
    Crossover,
    Mutation,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct SimulationSchedule;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(SimulationSchedule)
            .configure_sets(
                SimulationSchedule,
                (
                    SimulationSet::Objective,
                    SimulationSet::Fitness,
                    SimulationSet::Elitism,
                    SimulationSet::Selection,
                    SimulationSet::Crossover,
                    SimulationSet::Mutation,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                simulation_state::toggle_pause_simulation
                    .run_if(input_just_pressed(KeyCode::Space)),
            )
            .add_plugins((
                simulation_state::SimulationStatePlugin,
                selected_individuals::SelectedIndividualsPlugin,
                population::PopulationPlugin,
                evolutionary_steps::EvolutionaryStepsPlugin,
                fixed_timestep::FixedTimestepPlugin,
                generation_counter::GenerationCounterPlugin,
                early_stop::EarlyStopPlugin,
                fitness_diagnostics::FitnessDiagnosticsPlugin,
                simulation_timer::SimulationTimerPlugin,
            ));
    }
}
