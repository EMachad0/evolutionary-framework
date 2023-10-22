use bevy::prelude::*;

use crate::board::BoardPosition;
use crate::maze::Maze;
use evolutionary_framework::camera::MainCamera;

const BOARD_BORDER: f32 = 10.;

#[derive(Default, Debug, Component, Reflect)]
pub struct Board {
    pub world_size: Vec2,
    pub virtual_size: Vec2,
}

impl Board {
    pub fn cell_size(&self) -> Vec2 {
        self.world_size / self.virtual_size
    }
}

pub fn spawn_board(world: &mut World) {
    world.spawn((
        Name::new("Board"),
        Board::default(),
        VisibilityBundle::default(),
        TransformBundle::default(),
    ));
}

pub fn update_board_virtual_size(mut board_query: Query<&mut Board>, maze: Res<Maze>) {
    debug!("update_board_virtual_size");
    let mut board = board_query.single_mut();
    board.virtual_size = Vec2::new(maze.width as f32, maze.height as f32);
}
pub fn update_board_world_size(
    mut board_query: Query<(&mut Transform, &mut Board)>,
    cameras: Query<&Camera, (With<MainCamera>, Changed<Camera>)>,
) {
    let Ok(camera) = cameras.get_single() else {
        return;
    };
    let (cam_width, cam_height): (f32, f32) = camera.logical_viewport_size().unwrap().into();

    let board_width = cam_width - BOARD_BORDER;
    let board_height = cam_height - BOARD_BORDER;

    let (mut transform, mut board) = board_query.single_mut();
    transform.translation = Vec3::new(
        -board_width / 2.,
        -board_height / 2.,
        transform.translation.z,
    );
    board.world_size = Vec2::new(board_width, board_height);
}

pub fn update_board_children(
    board_query: Query<(&Board, &Children), Changed<Board>>,
    mut query: Query<(&mut Transform, &mut Sprite, &BoardPosition)>,
) {
    let Ok((board, children)) = board_query.get_single() else {
        return;
    };
    let cell_size = board.cell_size();
    for entity in children {
        let (mut transform, mut sprite, board_position) = query.get_mut(*entity).unwrap();
        let BoardPosition { x, y } = board_position;
        transform.translation = Vec3::from((
            Vec2::new(cell_size.x * *x as f32, cell_size.y * *y as f32),
            transform.translation.z,
        ));
        sprite.custom_size = Some(cell_size);
    }
}
