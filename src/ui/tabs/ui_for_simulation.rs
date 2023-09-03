use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;

use crate::camera::MainCamera;
use crate::ui::tabs::tab_tree::TAB_BAR_HEIGHT;

pub fn ui_for_simulation(world: &mut World, ui: &mut egui::Ui) {
    let mut viewport_rect = ui.clip_rect();
    viewport_rect.min = egui::pos2(viewport_rect.min.x, viewport_rect.min.y + TAB_BAR_HEIGHT);

    let Ok(window) = world.query_filtered::<&mut Window, With<PrimaryWindow>>().get_single(world) else {
        return;
    };

    let egui_settings = world.resource::<bevy_egui::EguiSettings>();
    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = viewport_rect.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = viewport_rect.size() * scale_factor as f32;

    let mut cam = world
        .query_filtered::<&mut Camera, With<MainCamera>>()
        .single_mut(world);
    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        depth: 0.0..1.0,
    });
}
