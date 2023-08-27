use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use evolutionary_framework::GameState;

use crate::cartesian_plane::plane::Plane;
use crate::fitness;
use crate::function::Function;
use evolutionary_framework::simulation::population::genes::{Bool, Gene};
use evolutionary_framework::simulation::population::individual::Individual;

pub struct IndividualPlugin;

impl Plugin for IndividualPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                (spawn_individuals).run_if(in_state(GameState::Playing)),
                (update_individuals).run_if(
                    in_state(GameState::Playing).and_then(any_with_component::<Function>()),
                ),
            ),
        );
    }
}

pub fn spawn_individuals(
    mut commands: Commands,
    individuals: Query<Entity, Added<Individual>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in individuals.into_iter() {
        commands
            .get_entity(entity)
            .unwrap()
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            });
    }
}

pub fn update_individuals(
    mut individuals: Query<(&mut Transform, &Gene<Bool>)>,
    function: Query<&Function>,
    plane: Query<&Plane>,
) {
    let Plane { scale } = *plane.single();
    let function = function.single();
    let x_domain = function.x_domain;
    for (mut transform, gene) in individuals.iter_mut() {
        let x = fitness::gene_to_value(gene) * (x_domain.1 - x_domain.0) + x_domain.0;
        let y = (function.f)(x);
        transform.translation = Vec3::new(x * scale, y * scale, transform.translation.z);
    }
}
