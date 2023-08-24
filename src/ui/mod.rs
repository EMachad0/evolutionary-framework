pub mod config;
pub mod inspector;
pub mod menu;

use bevy::prelude::*;

use crate::ui::config::UiConfig;

#[derive(Debug, Default)]
pub struct UiPlugin {
    config: UiConfig,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config)
            .add_plugins((inspector::InspectorPlugin, menu::MenuPlugin));
    }
}
