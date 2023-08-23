use bevy::prelude::*;
use bevy::sprite::Anchor;
use evolutionary_framework::simulation::population::genes::{Gene, Perm};
use evolutionary_framework::simulation::population::Individual;

use crate::board::Board;
use crate::board_position::BoardPosition;
use crate::selected_individual::SelectedIndividual;

#[derive(Debug, Component, Reflect)]
pub struct Queen;

pub fn spawn_queens(mut commands: Commands, board: Res<Board>, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("textures/crown.png");
    for i in 0..board.size {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BISQUE,
                    custom_size: Some(Vec2::new(board.cell_size, board.cell_size)),
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_xyz(0., 0., 1.),
                texture: texture.clone(),
                ..default()
            },
            BoardPosition { x: i, y: i },
            Queen,
            Name::new(format!("Queen {i}")),
        ));
    }
}

pub fn queens_from_selected_individual(
    mut queens: Query<&mut BoardPosition, With<Queen>>,
    individuals: Query<&Individual<Perm>>,
    selected: Res<SelectedIndividual>,
) {
    if let Some(entity) = selected.0 {
        let individual = individuals.get(entity).unwrap();
        let perm = individual.0.first().unwrap().get();

        for ((x, y), mut board_position) in perm.iter().enumerate().zip(queens.iter_mut()) {
            *board_position = BoardPosition { x, y: *y as usize };
        }
    }
}
