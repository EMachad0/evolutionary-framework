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
}

pub fn spawn_board(mut commands: Commands, config: Res<Config>, windows: Query<&Window>) {
    let window = windows.single();
    let size = config.population.dim;

    let board_size = window.width().min(window.height()) - BOARD_BORDER;
    let rect = Rect::new(
        -board_size / 2.,
        -board_size / 2.,
        board_size / 2.,
        board_size / 2.,
    );
    let cell_size = board_size / size as f32;

    let _cells = (0..size * size)
        .map(|i| {
            let x = i / size;
            let y = i % size;

            let color = if (x + y) % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            };

            commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(cell_size, cell_size)),
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        ..default()
                    },
                    BoardPosition { x, y },
                    Name::new(format!("Board Cell {x} {y}")),
                ))
                .id()
        })
        .collect::<Vec<_>>();

    commands.insert_resource(Board { rect, cell_size });
}

pub fn update_board_if_window_resize(
    config: Res<Config>,
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
        let cell_size = board_size / config.population.dim as f32;
        *board = Board { rect, cell_size };

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
