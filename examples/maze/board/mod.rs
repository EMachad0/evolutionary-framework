mod board_cells;
mod board_position;
mod spawn_board;

use bevy::prelude::*;

use crate::maze::Maze;
pub use board_position::BoardPosition;
use evolutionary_framework::GameState;
pub use spawn_board::Board;

const MAX_BOARD_DISPLAY_SIZE: usize = 32;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Board>()
            .register_type::<BoardPosition>()
            .add_systems(
                OnEnter(GameState::Playing),
                (spawn_board::spawn_board, board_cells::spawn_board_cells)
                    .chain()
                    .run_if(is_small_board_size),
            )
            .add_systems(
                PreUpdate,
                (
                    (
                        spawn_board::update_board_virtual_size.run_if(
                            in_state(GameState::Playing)
                                .and_then(any_with_component::<Board>())
                                .and_then(resource_changed::<Maze>()),
                        ),
                        spawn_board::update_board_world_size,
                    ),
                    (
                        spawn_board::update_board_children,
                        board_position::transform_from_board_position,
                    ),
                )
                    .chain()
                    .run_if(in_state(GameState::Playing).and_then(any_with_component::<Board>())),
            );
    }
}

pub fn is_small_board_size(maze: Res<Maze>) -> bool {
    maze.width * maze.height <= MAX_BOARD_DISPLAY_SIZE * MAX_BOARD_DISPLAY_SIZE
}
