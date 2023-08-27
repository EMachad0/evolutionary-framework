use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::cartesian_plane::plane::Plane;

#[derive(Debug, Default, Copy, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct BackgroundConfig {
    pub x_domain: (f32, f32),
    pub y_domain: (f32, f32),
}

#[derive(Debug, Component)]
pub struct PlaneBackground;

pub fn update_plane_background(
    mut commands: Commands,
    plane: Query<&Plane>,
    background: Query<Entity, With<PlaneBackground>>,
    background_config: Option<Res<BackgroundConfig>>,
) {
    if let Some(background_config) = background_config {
        let BackgroundConfig { x_domain, y_domain } = *background_config;
        let Plane { scale } = *plane.single();
        let background_entity = background.single();

        let mut children: Vec<Entity> = Vec::new();
        for i in x_domain.0.floor() as i32..=x_domain.1.ceil() as i32 {
            let sz = y_domain.1.ceil() - y_domain.0.floor();
            let entity = commands
                .spawn((
                    Name::new(format!("Horizontal line {i}")),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(1., sz * scale)),
                            color: Color::BLACK,
                            anchor: Anchor::BottomCenter,
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            i as f32 * scale,
                            y_domain.0.floor() * scale,
                            -1.,
                        ),
                        ..default()
                    },
                ))
                .id();
            children.push(entity);
        }
        for i in y_domain.0.floor() as i32..=y_domain.1.ceil() as i32 {
            let sz = x_domain.1.ceil() - x_domain.0.floor();
            let entity = commands
                .spawn((
                    Name::new(format!("Horizontal line {i}")),
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(sz * scale, 1.)),
                            color: Color::BLACK,
                            anchor: Anchor::CenterLeft,
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            x_domain.0.floor() * scale,
                            i as f32 * scale,
                            -1.,
                        ),
                        ..default()
                    },
                ))
                .id();
            children.push(entity);
        }

        commands
            .get_entity(background_entity)
            .unwrap()
            .replace_children(&children);
        commands.remove_resource::<BackgroundConfig>();
    }
}
