pub mod menu;
pub mod tabs;
pub mod ui_config;

use bevy::prelude::*;

use crate::ui::ui_config::UiConfig;

#[derive(Debug, Default)]
pub struct UiPlugin {
    config: UiConfig,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config)
            .add_plugins((tabs::TabsPlugin, menu::MenuPlugin));
    }
}
