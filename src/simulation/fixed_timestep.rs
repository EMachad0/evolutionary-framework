use bevy::prelude::*;
use std::time::Duration;

use crate::config::Config;
use crate::simulation::simulation_state::is_simulation_paused;
use crate::simulation::SimulationSchedule;
use crate::GameState;

pub const DEFAULT_STEPS_PER_SECOND: f64 = 60.;

pub struct FixedTimestepPlugin;

impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SimulationStep>()
            .init_resource::<SimulationStep>()
            .add_systems(OnEnter(GameState::Playing), init_simulation_step)
            .add_systems(
                FixedUpdate,
                run_simulation_schedule
                    .run_if(in_state(GameState::Playing).and_then(not(is_simulation_paused))),
            )
            .add_systems(
                PostUpdate,
                update_fixed_time_from_simulation_step.run_if(resource_changed::<SimulationStep>()),
            );
    }
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct SimulationStep {
    pub steps_per_second: f64,
}

impl Default for SimulationStep {
    fn default() -> Self {
        Self {
            steps_per_second: DEFAULT_STEPS_PER_SECOND,
        }
    }
}

pub fn init_simulation_step(mut commands: Commands, config: Res<Config>) {
    commands.insert_resource(SimulationStep {
        steps_per_second: config.simulation.steps_per_second,
    });
}

pub fn update_fixed_time_from_simulation_step(
    mut fixed_time: ResMut<FixedTime>,
    step: Res<SimulationStep>,
) {
    fixed_time.period = Duration::from_secs_f64(1. / step.steps_per_second);
}

pub fn run_simulation_schedule(world: &mut World) {
    world.run_schedule(SimulationSchedule);
}
