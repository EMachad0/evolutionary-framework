use bevy::prelude::*;

use crate::config::Config;
use crate::simulation::SimulationSchedule;
use crate::GameState;

pub struct GenerationCounterPlugin;

impl Plugin for GenerationCounterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GenerationCounter>()
            .init_resource::<GenerationCounter>()
            .add_systems(OnEnter(GameState::Playing), init_generation_counter)
            .add_systems(SimulationSchedule, update_counter);
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct GenerationCounter {
    pub current: u64,
    pub target: u64,
}

impl GenerationCounter {
    pub fn new(target: u64) -> Self {
        Self {
            target,
            ..default()
        }
    }
}

pub fn init_generation_counter(world: &mut World) {
    let target = world.resource::<Config>().simulation.target_generation;
    world.insert_resource(GenerationCounter::new(target))
}

pub fn update_counter(mut counter: ResMut<GenerationCounter>) {
    counter.current += 1
}

pub fn counter_just_finished(counter: Res<GenerationCounter>) -> bool {
    counter.current == counter.target
}
