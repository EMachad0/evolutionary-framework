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
        app.add_systems(OnEnter(GameState::Playing), init_simulation_step)
            .add_systems(
                FixedUpdate,
                run_simulation_schedule
                    .run_if(in_state(GameState::Playing).and_then(not(is_simulation_paused))),
            );
    }
}

pub fn init_simulation_step(mut commands: Commands, config: Res<Config>) {
    commands.insert_resource(FixedTime::new(Duration::from_secs_f64(
        1. / config.simulation.steps_per_second,
    )));
}

pub fn run_simulation_schedule(world: &mut World) {
    world.run_schedule(SimulationSchedule);
}
