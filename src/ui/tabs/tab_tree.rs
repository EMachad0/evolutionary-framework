use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContext, EguiSet};
use bevy_inspector_egui::{bevy_inspector::ui_for_world, DefaultInspectorConfigPlugin};
use egui_dock::{DockArea, NodeIndex, Style, TabBarStyle, Tree};

use crate::simulation::generation_counter::GenerationCounter;
use crate::ui::tabs::ui_for_controls::{update_ui_state_generation_counter, ControlsUiState};
use crate::ui::tabs::{ui_for_controls, ui_for_individuals, ui_for_simulation};
use crate::GameState;

pub const TAB_BAR_HEIGHT: f32 = 20.;

pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultInspectorConfigPlugin)
            .add_plugins(bevy_egui::EguiPlugin)
            .register_type::<ControlsUiState>()
            .init_resource::<ControlsUiState>()
            .insert_resource(TabUiState::new())
            .add_systems(
                PostUpdate,
                (show_ui_system
                    .before(EguiSet::ProcessOutput)
                    .before(bevy::transform::TransformSystem::TransformPropagate),)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                update_ui_state_generation_counter.run_if(resource_added::<GenerationCounter>()),
            );
    }
}

#[derive(Debug)]
enum UiWindows {
    Simulation,
    Individuals,
    Inspector,
    Controls,
}

#[derive(Resource)]
struct TabUiState {
    tree: Tree<UiWindows>,
}

impl TabUiState {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![UiWindows::Simulation]);
        let [_game, inspector] = tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![UiWindows::Individuals, UiWindows::Inspector],
        );
        let [_inspector, _controls] = tree.split_below(inspector, 0.8, vec![UiWindows::Controls]);

        Self { tree }
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer { world };
        let style = {
            let mut tab_bar_style = TabBarStyle::from_egui(ctx.style().as_ref());
            tab_bar_style.height = TAB_BAR_HEIGHT;

            let mut style = Style::from_egui(ctx.style().as_ref());
            style.tab_bar = tab_bar_style;
            style
        };
        DockArea::new(&mut self.tree)
            .show_close_buttons(false)
            .draggable_tabs(false)
            .style(style)
            .show(ctx, &mut tab_viewer);
    }
}

struct TabViewer<'w> {
    world: &'w mut World,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = UiWindows;

    fn ui(&mut self, ui: &mut egui::Ui, window: &mut Self::Tab) {
        match window {
            UiWindows::Simulation => ui_for_simulation::ui_for_simulation(self.world, ui),
            UiWindows::Individuals => ui_for_individuals::ui_for_individuals(self.world, ui),
            UiWindows::Inspector => ui_for_world(self.world, ui),
            UiWindows::Controls => ui_for_controls::ui_for_controls(self.world, ui),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, UiWindows::Simulation)
    }
}

fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<TabUiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}
