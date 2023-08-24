use bevy::prelude::*;

#[derive(Debug, Default, Copy, Clone, Reflect)]
pub enum SelectionMode {
    #[default]
    Single,
    Many,
}

#[derive(Debug, Default, Copy, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct UiConfig {
    pub selection_mode: SelectionMode,
}
