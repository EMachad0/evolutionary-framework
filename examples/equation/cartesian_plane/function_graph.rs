use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use itertools::Itertools;

use crate::cartesian_plane::background::BackgroundConfig;
use crate::cartesian_plane::plane::Plane;
use crate::cartesian_plane::Graphs;
use crate::function::Function;

const NUM_OF_POINTS: usize = 1000;

#[derive(Debug, Component)]
pub struct FunctionGraph;

pub fn spawn_function_graph(
    mut commands: Commands,
    graphs: Query<Entity, With<Graphs>>,
    plane: Query<&Plane>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    function: Res<Function>,
) {
    let Function {
        f,
        x_domain,
        y_domain,
    } = *function;
    let Plane { scale } = *plane.single();
    let thickness = 2.;
    let entity = commands
        .spawn((
            Name::new("Objective Function"),
            FunctionGraph,
            TransformBundle::default(),
            VisibilityBundle::default(),
        ))
        .with_children(|parent| {
            let mut points = Vec::with_capacity(NUM_OF_POINTS);
            for i in 0..NUM_OF_POINTS {
                let step = (x_domain.1 - x_domain.0) / (NUM_OF_POINTS - 1) as f32;
                let x = x_domain.0 + step * i as f32;
                points.push(Vec2::new(x, (f)(x)));
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
