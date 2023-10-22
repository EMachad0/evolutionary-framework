use bevy::prelude::*;

use crate::board::Board;

#[derive(Debug, Copy, Clone, Component, Reflect)]
pub struct BoardPosition {
    pub x: usize,
    pub y: usize,
}

pub fn transform_from_board_position(
    board_query: Query<&Board>,
    mut query: Query<(&mut Transform, &BoardPosition), Changed<BoardPosition>>,
) {
    if query.is_empty() {
        return;
    }
    let board = board_query.single();
    let cell_size = board.cell_size();
    for (mut transform, board_position) in query.iter_mut() {
        let BoardPosition { x, y } = board_position;
        transform.translation = Vec3::from((
            Vec2::new(cell_size.x * *x as f32, cell_size.y * *y as f32),
            transform.translation.z,
        ));
    }
}
