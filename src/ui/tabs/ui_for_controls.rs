use bevy::prelude::*;

use crate::simulation::generation_counter::GenerationCounter;
use crate::simulation::simulation_state::SimulationState;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct ControlsUiState {
    generation_counter_target: String,
}

pub fn update_ui_state_generation_counter(
    mut ui_state: ResMut<ControlsUiState>,
    counter: Res<GenerationCounter>,
) {
    ui_state.generation_counter_target = counter.target.to_string();
}

pub fn ui_for_controls(world: &mut World, ui: &mut egui::Ui) {
    let cell = world.cell();

    let mut ui_state = cell.resource_mut::<ControlsUiState>();

    let horizontal_ratio = 0.6;
    let mut simulation_state = cell.resource_mut::<SimulationState>();
    ui.toggle_value(&mut simulation_state.paused, "Pause");

    let mut generation_counter = cell.resource_mut::<GenerationCounter>();

    ui.horizontal(|ui| {
        ui.add_sized(
            egui::Vec2::new(ui.available_width() * horizontal_ratio, 0.),
            egui::Label::new("Current Generation:"),
        );
        ui.add(egui::Label::new(generation_counter.current.to_string()));
    });

    ui.horizontal(|ui| {
        ui.add_sized(
            egui::Vec2::new(ui.available_width() * horizontal_ratio, 0.),
            egui::Label::new("Target Generation:"),
        );
        let generation_target_response = ui.add_sized(
            egui::Vec2::new(ui.available_width(), 0.),
            egui::TextEdit::singleline(&mut ui_state.generation_counter_target),
        );
        if generation_target_response.lost_focus() {
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                match ui_state.generation_counter_target.parse::<u32>() {
                    Ok(val) => generation_counter.target = val,
                    Err(_) => {
                        ui_state.generation_counter_target = generation_counter.target.to_string()
                    }
                }
            } else {
                ui_state.generation_counter_target = generation_counter.target.to_string();
            }
        }
    });
}
