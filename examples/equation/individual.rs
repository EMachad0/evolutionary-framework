use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::cartesian_plane::plane::Plane;
use crate::objective::EquationObjective;
use evolutionary_framework::simulation::population::individual::Individual;
use evolutionary_framework::simulation::SimulationSet;
use evolutionary_framework::GameState;

pub struct IndividualPlugin;

impl Plugin for IndividualPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            insert_individual_shape.after(SimulationSet::PopulationStart),
        )
        .add_systems(
            PostUpdate,
            (update_individual_shape.run_if(in_state(GameState::Playing)),),
        );
    }
}

pub fn insert_individual_shape(
    mut commands: Commands,
    individuals: Query<Entity, With<Individual>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in individuals.into_iter() {
        commands
            .get_entity(entity)
            .unwrap()
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::ORANGE)),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            });
    }
}

pub fn update_individual_shape(
    mut individuals: Query<(&mut Transform, &EquationObjective)>,
    plane: Query<&Plane>,
) {
    let Plane { scale } = *plane.single();
    for (mut transform, objective) in individuals.iter_mut() {
        let EquationObjective { x, y } = objective;
        transform.translation = Vec3::new(x * scale, y * scale, transform.translation.z);
    }
}
