use bevy::prelude::*;

pub struct SimulationStatePlugin;

impl Plugin for SimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SimulationState>()
            .init_resource::<SimulationState>();
    }
}

#[derive(Debug, Default, Copy, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct SimulationState {
    pub paused: bool,
}

pub fn is_simulation_paused(status: Res<SimulationState>) -> bool {
    status.paused
}

pub fn pause_simulation(mut status: ResMut<SimulationState>) {
    status.paused = true;
}

pub fn toggle_pause_simulation(mut status: ResMut<SimulationState>) {
    status.paused = !status.paused;
}
