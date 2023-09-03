pub mod early_stop;
pub mod evolutionary_steps;
pub mod fixed_timestep;
pub mod generation_counter;
pub mod population;
pub mod selected_individuals;
pub mod simulation_state;

use bevy::ecs::schedule::ScheduleLabel;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, SystemSet)]
pub enum SimulationSet {
    PopulationStart,
    Elitism,
    Selection,
    Crossover,
    Mutation,
    Fitness,
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
                simulation_state::pause_simulation.run_if(input_just_pressed(KeyCode::Space)),
            )
            .add_systems(
                SimulationSchedule,
                simulation_state::pause_simulation
                    .after(generation_counter::update_counter)
                    .run_if(generation_counter::counter_just_finished),
            )
            .add_plugins((
                simulation_state::SimulationStatePlugin,
                selected_individuals::SelectedIndividualsPlugin,
                population::PopulationPlugin,
                evolutionary_steps::EvolutionaryStepsPlugin,
                fixed_timestep::FixedTimestepPlugin,
                generation_counter::GenerationCounterPlugin,
                early_stop::EarlyStopPlugin,
            ));
    }
}
