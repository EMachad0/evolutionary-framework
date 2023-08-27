use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

use evolutionary_framework::camera::MainCamera;
use evolutionary_framework::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin)
            .add_systems(OnEnter(GameState::Playing), add_camera_pancam);
    }
}

pub fn add_camera_pancam(mut commands: Commands, camera: Query<Entity, With<MainCamera>>) {
    commands
        .get_entity(camera.single())
        .unwrap()
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Right],
            enabled: true,
            zoom_to_cursor: true,
            min_scale: 0.01,
            max_scale: None,
            ..default()
        });
}
