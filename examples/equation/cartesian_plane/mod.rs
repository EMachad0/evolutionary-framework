pub mod axis;
pub mod background;
pub mod function_graph;
pub mod plane;

use bevy::prelude::*;

use crate::cartesian_plane::background::BackgroundConfig;
use crate::cartesian_plane::function_graph::FunctionGraph;
use crate::cartesian_plane::plane::Plane;
use axis::AxisConfig;
use evolutionary_framework::despawn::despawn;
use evolutionary_framework::GameState;

pub struct CartesianPlanePlugin;

impl Plugin for CartesianPlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), plane::spawn_plane)
            .add_systems(
                PostUpdate,
                (
                    axis::update_plane_axis.run_if(resource_exists::<AxisConfig>()),
                    background::update_plane_background
                        .run_if(resource_exists::<BackgroundConfig>()),
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PreUpdate,
                function_graph::spawn_function_graph.run_if(
                    in_state(GameState::Playing)
                        .and_then(any_with_component::<Graphs>())
                        .and_then(not(any_with_component::<FunctionGraph>())),
                ),
            )
            .add_systems(OnExit(GameState::Playing), despawn::<Plane>);
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Graphs;
