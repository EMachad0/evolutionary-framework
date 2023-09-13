use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::simulation::simulation_state::is_simulation_paused;
use crate::GameState;

pub struct SimulationTimerPlugin;

impl Plugin for SimulationTimerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SimulationTimer>()
            .init_resource::<SimulationTimer>()
            .add_systems(
                PostUpdate,
                update_simulation_timer
                    .run_if(in_state(GameState::Playing).and_then(not(is_simulation_paused))),
            )
            .add_systems(OnEnter(GameState::Playing), reset_simulation_timer);
    }
}

#[derive(Debug, Default, Deref, DerefMut, Resource, Reflect)]
#[reflect(Resource)]
pub struct SimulationTimer(Stopwatch);

pub fn update_simulation_timer(mut timer: ResMut<SimulationTimer>, time: Res<Time>) {
    timer.tick(time.delta());
}

pub fn reset_simulation_timer(mut timer: ResMut<SimulationTimer>) {
    timer.unpause();
    timer.reset();
}
