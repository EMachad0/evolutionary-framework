use bevy::prelude::*;

use evolutionary_framework::selected_individual::SelectedIndividuals;
use evolutionary_framework::simulation::population::genes::{Gene, Perm};
use evolutionary_framework::simulation::population::Individual;

#[derive(Default, Debug, Component, Reflect)]
pub struct Fitness(i32);

pub fn calc_fitness(mut commands: Commands, individuals: Query<(Entity, &Individual<Perm>)>) {
    for (entity, individual) in individuals.iter() {
        let perm = individual.0.first().unwrap().get();
        let n = perm.len();
        let mut diagonals1: Vec<i32> = vec![0; n * 2];
        let mut diagonals2: Vec<i32> = vec![0; n * 2];
        let mut fitness = 0;

        for (x, y) in perm.iter().enumerate() {
            let y = *y as usize;
            fitness += diagonals1[n + x - y];
            fitness += diagonals2[x + y];
            diagonals1[n + x - y] += 1;
            diagonals2[x + y] += 1;
        }

        commands.entity(entity).insert(Fitness(fitness));
    }
}

#[derive(Component)]
pub struct FitnessUiText;

pub fn spawn_fitness_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::RED,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        FitnessUiText,
    ));
}

pub fn update_fitness_ui(
    mut query: Query<&mut Text, With<FitnessUiText>>,
    selected: Res<SelectedIndividuals>,
    individuals: Query<&Fitness, With<Individual<Perm>>>,
) {
    if let Some(entity) = selected.0 {
        let fitness = individuals.get(entity).unwrap();
        for mut text in &mut query {
            text.sections[0].value = format!("{}", fitness.0);
        }
    }
}
