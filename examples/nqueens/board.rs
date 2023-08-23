use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::WindowResized;

use crate::board_position::BoardPosition;
use evolutionary_framework::config::Config;

const BOARD_BORDER: f32 = 10.;

#[derive(Default, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct Board {
    pub rect: Rect,
    pub cell_size: f32,
    pub size: usize,
}

pub fn spawn_board(world: &mut World) {
    let (width, height) = {
        let window = world.query::<&Window>().single(world);
        (window.width(), window.height())
    };
    let config = world.resource::<Config>();

    let size = config.population.dim;
    let pixel_size = width.min(height) - BOARD_BORDER;
    let rect = Rect::new(
        -pixel_size / 2.,
        -pixel_size / 2.,
        pixel_size / 2.,
        pixel_size / 2.,
    );
    let cell_size = pixel_size / size as f32;

    world.insert_resource(Board {
        rect,
        cell_size,
        size,
    });
}

pub fn spawn_board_cells(mut commands: Commands, board: Res<Board>) {
    for i in 0..board.size * board.size {
        let x = i / board.size;
        let y = i % board.size;

        let color = if (x + y) % 2 == 0 {
            Color::WHITE
        } else {
            Color::BLACK
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(board.cell_size, board.cell_size)),
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                ..default()
            },
            BoardPosition { x, y },
            Name::new(format!("Board Cell {x} {y}")),
        ));
    }
}

pub fn update_board_if_window_resize(
    mut board: ResMut<Board>,
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<(&mut Transform, &mut Sprite, &BoardPosition)>,
) {
    if let Some(e) = resize_reader.iter().last() {
        let board_size = e.width.min(e.height) - BOARD_BORDER;
        let rect = Rect::new(
            -board_size / 2.,
            -board_size / 2.,
            board_size / 2.,
            board_size / 2.,
        );
        let size = board.size;
        let cell_size = board_size / size as f32;
        *board = Board {
            rect,
            cell_size,
            size,
        };

        for (mut transform, mut sprite, board_position) in query.iter_mut() {
            let BoardPosition { x, y } = board_position;
            transform.translation = Vec3::from((
                rect.min + Vec2::new(cell_size * *x as f32, cell_size * *y as f32),
                0.,
            ));
            sprite.custom_size = Some(Vec2::new(cell_size, cell_size));
        }
    }
}
