use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::board::Board;
use crate::board_position::BoardPosition;
use crate::loading::textures::TextureAssets;
use evolutionary_framework::simulation::population::genes::{Gene, GeneCod, Perm};
use evolutionary_framework::simulation::selected_individuals::SelectedIndividuals;

#[derive(Debug, Component, Reflect)]
pub struct Queen;

pub fn spawn_queens(mut commands: Commands, board: Res<Board>, textures: Res<TextureAssets>) {
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
                texture: textures.crown.clone(),
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
    individuals: Query<&Gene<Perm>>,
    selected: Res<SelectedIndividuals>,
) {
    if let Some(entity) = selected.single() {
        let individual = individuals.get(*entity).unwrap();
        let perm = individual.get().get();

        for ((x, y), mut board_position) in perm.iter().enumerate().zip(queens.iter_mut()) {
            *board_position = BoardPosition { x, y: *y as usize };
        }
    }
}
