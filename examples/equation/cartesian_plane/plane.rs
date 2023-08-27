use bevy::prelude::*;

use crate::cartesian_plane::axis::Axis;
use crate::cartesian_plane::background::PlaneBackground;
use crate::cartesian_plane::Graphs;

#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct Plane {
    pub scale: f32,
}

impl Plane {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }
}

pub fn spawn_plane(mut commands: Commands) {
    let scale = 50.;
    commands
        .spawn((
            Plane::new(scale),
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(0., 0., -1.)),
            Name::new("Plane"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Axis,
                Name::new("Axis"),
                VisibilityBundle::default(),
                TransformBundle::default(),
            ));
            parent.spawn((
                PlaneBackground,
                Name::new("Plane Background"),
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_xyz(0., 0., -1.)),
            ));
            parent.spawn((
                Graphs,
                Name::new("Graphs"),
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_xyz(0., 0., 1.)),
            ));
        });
}
