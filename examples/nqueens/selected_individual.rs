use bevy::prelude::*;

use evolutionary_framework::simulation::population::genes::Perm;
use evolutionary_framework::simulation::population::Individual;

#[derive(Default, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct SelectedIndividual(pub Option<Entity>);

pub fn select_individual_from_kb(
    mut selection: ResMut<SelectedIndividual>,
    individuals: Query<Entity, With<Individual<Perm>>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Key1) {
        selection.0 = individuals.iter().next();
    }
    if keys.just_pressed(KeyCode::Key2) {
        selection.0 = individuals.iter().nth(1);
    }
    if keys.just_pressed(KeyCode::Key3) {
        selection.0 = individuals.iter().nth(2);
    }
    if keys.just_pressed(KeyCode::Key4) {
        selection.0 = individuals.iter().nth(3);
    }
    if keys.just_pressed(KeyCode::Key5) {
        selection.0 = individuals.iter().nth(4);
    }
    if keys.just_pressed(KeyCode::Key6) {
        selection.0 = individuals.iter().nth(5);
    }
    if keys.just_pressed(KeyCode::Key7) {
        selection.0 = individuals.iter().nth(6);
    }
    if keys.just_pressed(KeyCode::Key8) {
        selection.0 = individuals.iter().nth(7);
    }
    if keys.just_pressed(KeyCode::Key9) {
        selection.0 = individuals.iter().nth(8);
    }
    if keys.just_pressed(KeyCode::Key0) {
        selection.0 = individuals.iter().nth(9);
    }
}
