pub mod fixed_timestep;
pub mod generation_counter;
pub mod population;
pub mod selected_individuals;
pub mod selection;
pub mod simulation_state;

use bevy::ecs::schedule::ScheduleLabel;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, SystemSet)]
pub enum SimulationSet {
    PopulationStart,
    Elitism,
    Selection,
    Fitness,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct SelectionSchedule;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(SelectionSchedule)
            .configure_sets(
                SelectionSchedule,
                (
                    SimulationSet::Elitism,
                    SimulationSet::Selection,
                    SimulationSet::Fitness,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                simulation_state::pause_simulation.run_if(input_just_pressed(KeyCode::Space)),
            )
            .add_systems(
                SelectionSchedule,
                simulation_state::pause_simulation
                    .after(generation_counter::update_counter)
                    .run_if(generation_counter::counter_just_finished),
            )
            .add_plugins((
                simulation_state::SimulationStatePlugin,
                selected_individuals::SelectedIndividualsPlugin,
                population::PopulationPlugin,
                selection::SelectionPlugin,
                fixed_timestep::FixedTimestepPlugin,
                generation_counter::GenerationCounterPlugin,
            ));
    }
}
