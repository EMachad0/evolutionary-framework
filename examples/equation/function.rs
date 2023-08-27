use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use itertools::Itertools;

use crate::cartesian_plane::background::BackgroundConfig;
use crate::cartesian_plane::plane::Plane;
use crate::cartesian_plane::Graphs;
use evolutionary_framework::GameState;

const NUM_OF_POINTS: usize = 1000;

#[derive(Debug, Copy, Clone, Component)]
pub struct Function {
    pub f: fn(f32) -> f32,
    pub x_domain: (f32, f32),
    pub y_domain: (f32, f32),
}

pub fn objetive_function(x: f32) -> f32 {
    (20. * x).cos() - (x.abs() / 2.) + (x * x * x / 4.)
}

pub struct FunctionPlugin;

impl Plugin for FunctionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            spawn_objective_function.run_if(
                in_state(GameState::Playing)
                    .and_then(any_with_component::<Graphs>())
                    .and_then(run_once()),
            ),
        );
    }
}

pub fn spawn_objective_function(
    mut commands: Commands,
    graphs: Query<Entity, With<Graphs>>,
    plane: Query<&Plane>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Plane { scale } = *plane.single();
    let x_domain = (-2., 2.);
    let y_domain = (-4., 2.);
    let thickness = 2.;
    let entity = commands
        .spawn((
            Name::new("Objective Function"),
            Function {
                f: objetive_function,
                x_domain,
                y_domain,
            },
            TransformBundle::default(),
            VisibilityBundle::default(),
        ))
        .with_children(|parent| {
            let mut points = Vec::with_capacity(NUM_OF_POINTS);
            for i in 0..NUM_OF_POINTS {
                let step = (x_domain.1 - x_domain.0) / (NUM_OF_POINTS - 1) as f32;
                let x = x_domain.0 + step * i as f32;
                points.push(Vec2::new(x, objetive_function(x)));
            }

            for Vec2 { x, y } in points.iter() {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(thickness / 2.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: Transform::from_translation(Vec3::new(x * scale, y * scale, 0.)),
                    ..default()
                });
            }

            for (p0, p1) in points.iter().tuple_windows::<(_, _)>() {
                let sz = p0.distance(*p1);
                let angle = -(*p1 - *p0).angle_between(Vec2::X);
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::PURPLE,
                        anchor: Anchor::CenterLeft,
                        custom_size: Some(Vec2::new(sz * scale, thickness)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        p0.x * scale,
                        p0.y * scale,
                        0.,
                    ))
                    .with_rotation(Quat::from_rotation_z(angle)),
                    ..default()
                });
            }
        })
        .id();

    let parent = graphs.single();
    commands.get_entity(parent).unwrap().add_child(entity);
    commands.insert_resource(BackgroundConfig { x_domain, y_domain });
}
