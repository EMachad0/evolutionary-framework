use bevy::prelude::*;

use crate::board::Board;

#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct BoardPosition {
    pub x: usize,
    pub y: usize,
}

pub fn transform_from_board_position(
    mut query: Query<(&mut Transform, &BoardPosition), Changed<BoardPosition>>,
    board: Res<Board>,
) {
    for (mut transform, board_position) in query.iter_mut() {
        let BoardPosition { x, y } = board_position;
        transform.translation = Vec3::from((
            board.rect.min + Vec2::new(board.cell_size * *x as f32, board.cell_size * *y as f32),
            0.,
        ));
    }
}
