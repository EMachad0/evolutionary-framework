use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

#[derive(Default, Component)]
pub struct UiCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), UiCamera));
}
