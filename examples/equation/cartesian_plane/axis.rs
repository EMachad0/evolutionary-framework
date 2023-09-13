use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use std::f32::consts::PI;

use crate::cartesian_plane::plane::Plane;

#[derive(Debug, Default, Copy, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct AxisConfig {
    pub thickness: f32,
    pub alpha: f32,
    pub length: f32,
}

#[derive(Debug, Component)]
pub struct Axis;

pub fn update_plane_axis(
    mut commands: Commands,
    axis_config: ResMut<AxisConfig>,
    axis: Query<Entity, With<Axis>>,
    plane: Query<&Plane>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = axis.single();

    let AxisConfig {
        thickness,
        alpha,
        length,
    } = *axis_config;

    let Plane { scale } = *plane.single();

    let x_axis_color = Color::RED.with_a(alpha);
    let y_axis_color = Color::GREEN.with_a(alpha);

    let x_axis_entity = commands
        .spawn((
            Name::new("X axis"),
            SpriteBundle {
                sprite: Sprite {
                    color: x_axis_color,
                    custom_size: Some(Vec2::new(length * scale, thickness)),
                    anchor: Anchor::CenterLeft,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(2. * thickness, 3).into())
                    .into(),
                material: materials.add(ColorMaterial::from(x_axis_color)),
                transform: Transform::from_xyz(length * scale, 0., 0.)
                    .with_rotation(Quat::from_rotation_z(-PI / 2.)),
                ..default()
            });
        })
        .id();
    let y_axis_entity = commands
        .spawn((
            Name::new("Y axis"),
            SpriteBundle {
                sprite: Sprite {
                    color: y_axis_color,
                    custom_size: Some(Vec2::new(thickness, length * scale)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(2. * thickness, 3).into())
                    .into(),
                material: materials.add(ColorMaterial::from(y_axis_color)),
                transform: Transform::from_xyz(0., length * scale, 0.),
                ..default()
            });
        })
        .id();
    commands
        .get_entity(entity)
        .unwrap()
        .replace_children(&[x_axis_entity, y_axis_entity]);
    commands.remove_resource::<AxisConfig>();
}
