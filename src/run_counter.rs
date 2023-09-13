use bevy::prelude::*;

use crate::config::{Config, ConfigSet};
use crate::GameState;

pub struct RunCounterPlugin;

impl Plugin for RunCounterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RunCounter>()
            .init_resource::<RunCounter>()
            .add_systems(
                OnExit(GameState::Loading),
                init_run_counter.after(ConfigSet),
            )
            .add_systems(OnExit(GameState::Playing), update_counter);
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct RunCounter {
    pub current: u64,
    pub target: u64,
}

impl RunCounter {
    pub fn new(target: u64) -> Self {
        Self {
            target,
            ..default()
        }
    }
}

pub fn init_run_counter(world: &mut World) {
    let target = world.resource::<Config>().simulation.runs;
    world.insert_resource(RunCounter::new(target))
}

pub fn update_counter(mut counter: ResMut<RunCounter>) {
    counter.current += 1
}

pub fn counter_finished(counter: Res<RunCounter>) -> bool {
    counter.current >= counter.target
}
