use bevy::prelude::*;
use bevy::sprite::Anchor;
use itertools::Itertools;

use crate::board::{Board, BoardPosition};
use crate::objective::MazeObjective;
use evolutionary_framework::simulation::population::individual::Individual;
use evolutionary_framework::simulation::SimulationSet;
use evolutionary_framework::GameState;

pub struct IndividualPlugin;

impl Plugin for IndividualPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            insert_individual_position.after(SimulationSet::PopulationStart),
        )
        .add_systems(
            PostUpdate,
            (update_individual_position.run_if(in_state(GameState::Playing)),),
        );
    }
}

pub fn insert_individual_position(
    mut commands: Commands,
    board_query: Query<Entity, With<Board>>,
    individuals: Query<Entity, With<Individual>>,
) {
    for entity in individuals.into_iter() {
        commands.get_entity(entity).unwrap().insert((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
            BoardPosition::default(),
        ));
    }
    let board = board_query.single();
    let children = individuals.iter().collect_vec();
    commands.get_entity(board).unwrap().push_children(&children);
}

pub fn update_individual_position(mut individuals: Query<(&mut BoardPosition, &MazeObjective)>) {
    for (mut position, objective) in individuals.iter_mut() {
        let (x, y) = objective.position;
        *position = BoardPosition { x, y }
    }
}
