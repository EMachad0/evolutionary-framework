pub mod axis;
pub mod background;
pub mod plane;

use bevy::prelude::*;

use axis::AxisConfig;
use evolutionary_framework::GameState;

pub struct CartesianPlanePlugin;

impl Plugin for CartesianPlanePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AxisConfig {
            thickness: 2.,
            alpha: 0.3,
            length: 1.,
        })
        .add_systems(OnEnter(GameState::Playing), plane::spawn_plane)
        .add_systems(
            PostUpdate,
            (axis::update_plane_axis, background::update_plane_background)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Graphs;
