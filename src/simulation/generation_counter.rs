use bevy::prelude::*;

use crate::simulation::SimulationSchedule;

pub struct GenerationCounterPlugin;

impl Plugin for GenerationCounterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GenerationCounter>()
            .init_resource::<GenerationCounter>()
            .add_systems(SimulationSchedule, update_counter);
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct GenerationCounter {
    pub current: u32,
    pub target: u32,
}

pub fn update_counter(mut counter: ResMut<GenerationCounter>) {
    counter.current += 1
}

pub fn counter_just_finished(counter: Res<GenerationCounter>) -> bool {
    counter.current == counter.target
}
