use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContext, EguiSet};
use bevy_inspector_egui::{
    bevy_inspector::{hierarchy::SelectedEntities, ui_for_world},
    DefaultInspectorConfigPlugin,
};
use egui_dock::{DockArea, NodeIndex, Style, TabBarStyle, Tree};
use itertools::Itertools;

use crate::camera::MainCamera;
use crate::simulation::population::fitness::Fitness;
use crate::ui::selected_individuals::SelectedIndividuals;
use crate::ui::ui_config::SelectionMode;
use crate::GameState;

const TAB_BAR_HEIGHT: f32 = 20.;

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultInspectorConfigPlugin)
            .add_plugins(bevy_egui::EguiPlugin)
            .insert_resource(TabUiState::new())
            .add_systems(
                PostUpdate,
                (
                    show_ui_system
                        .before(EguiSet::ProcessOutput)
                        .before(bevy::transform::TransformSystem::TransformPropagate),
                    (set_camera_viewport, sync_ui_selected_individuals)
                        .after(show_ui_system)
                        .run_if(resource_changed::<TabUiState>()),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Debug)]
enum UiWindows {
    Simulation,
    Individuals,
    Inspector,
}

#[derive(Resource)]
struct TabUiState {
    tree: Tree<UiWindows>,
    viewport_rect: egui::Rect,
    selected_entities: SelectedEntities,
}

impl TabUiState {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![UiWindows::Simulation]);
        let [_game, _inspector] = tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![UiWindows::Individuals, UiWindows::Inspector],
        );

        Self {
            tree,
            selected_entities: SelectedEntities::default(),
            viewport_rect: egui::Rect::NOTHING,
        }
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let mut tab_viewer = TabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
        };
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
    selected_entities: &'w mut SelectedEntities,
    viewport_rect: &'w mut egui::Rect,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = UiWindows;

    fn ui(&mut self, ui: &mut egui::Ui, window: &mut Self::Tab) {
        match window {
            UiWindows::Simulation => {
                let mut viewport_rect = ui.clip_rect();
                viewport_rect.min =
                    egui::pos2(viewport_rect.min.x, viewport_rect.min.y + TAB_BAR_HEIGHT);
                *self.viewport_rect = viewport_rect;
            }
            UiWindows::Individuals => ui_for_individuals(self.world, ui, self.selected_entities),
            UiWindows::Inspector => {
                ui_for_world(self.world, ui);
            }
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, UiWindows::Simulation)
    }
}

fn ui_for_individuals(
    world: &mut World,
    ui: &mut egui::Ui,
    selected_entities: &mut SelectedEntities,
) {
    world
        .query::<(Entity, &Name, &Fitness)>()
        .iter(world)
        .sorted_by(|(_, _, f1), (_, _, f2)| f1.get().partial_cmp(&f2.get()).unwrap().reverse())
        .for_each(|(entity, name, fitness)| {
            let text = format!("{} ({:.2})", name, fitness.get());
            if ui
                .selectable_label(selected_entities.contains(entity), text)
                .clicked()
            {
                let ui_config = world.resource::<crate::ui::ui_config::UiConfig>();
                match ui_config.selection_mode {
                    SelectionMode::Single => selected_entities.select_replace(entity),
                    SelectionMode::Many => selected_entities.select_maybe_add(entity, true),
                };
            }
        });
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

fn set_camera_viewport(
    ui_state: Res<TabUiState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
) {
    let mut cam = cameras.single_mut();
    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor as f32;

    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        depth: 0.0..1.0,
    });
}

fn sync_ui_selected_individuals(
    ui_state: Res<TabUiState>,
    mut selection: ResMut<SelectedIndividuals>,
) {
    let selected = ui_state.selected_entities.iter().collect::<Vec<_>>();
    *selection.get_mut() = selected;
}
