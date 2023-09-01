pub mod population;
pub mod selection;

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
                run_simulation_step.run_if(input_just_pressed(KeyCode::Space)),
            )
            .add_plugins((population::PopulationPlugin, selection::SelectionPlugin));
    }
}

pub fn run_simulation_step(world: &mut World) {
    world.run_schedule(SelectionSchedule);
}
