use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::board::{Board, BoardPosition};
use crate::maze::Maze;

pub fn spawn_board_cells(
    mut commands: Commands,
    maze: Res<Maze>,
    board_query: Query<Entity, With<Board>>,
) {
    let mut cells: Vec<Entity> = Vec::new();
    for i in 0..maze.height {
        for j in 0..maze.width {
            let y = i;
            let x = j;

            let color = match maze[i][j] {
                0 => Color::BLACK,
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::RED,
                _ => unreachable!(),
            };

            let entity = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color,
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        ..default()
                    },
                    BoardPosition { x, y },
                    Name::new(format!("Board Cell {x} {y}")),
                ))
                .id();
            cells.push(entity);
        }
    }

    let board = board_query.single();
    commands.entity(board).push_children(&cells);
}
