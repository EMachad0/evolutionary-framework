use bevy::prelude::*;

pub mod camera;

use camera::CameraPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraPlugin);
    }
}
