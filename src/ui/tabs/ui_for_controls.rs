use crate::GameState;
use bevy::prelude::*;

use crate::simulation::fixed_timestep::SimulationStep;
use crate::simulation::generation_counter::GenerationCounter;
use crate::simulation::simulation_state::{SimulationState, SimulationStateType};
use crate::simulation::simulation_timer::SimulationTimer;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct ControlsUiState {
    generation_counter_target: String,
    steps_per_second: String,
}

pub fn update_ui_state_generation_counter(
    mut ui_state: ResMut<ControlsUiState>,
    counter: Res<GenerationCounter>,
) {
    ui_state.generation_counter_target = counter.target.to_string();
}

pub fn update_ui_state_steps_per_second(
    mut ui_state: ResMut<ControlsUiState>,
    step: Res<SimulationStep>,
) {
    ui_state.steps_per_second = format!("{:.2}", step.steps_per_second);
}

pub fn ui_for_controls(world: &mut World, ui: &mut egui::Ui) {
    let cell = world.cell();

    let mut ui_state = cell.resource_mut::<ControlsUiState>();

    let horizontal_ratio = 0.6;

    ui.horizontal(|ui| {
        let mut state = cell.resource_mut::<SimulationState>();
        ui.label("State: ");
        ui.label(state.get().to_string());
        if ui.button("Run").clicked() {
            state.set(SimulationStateType::Running);
        }
        if ui.button("Pause").clicked() {
            state.set(SimulationStateType::Paused);
        }
        if ui.button("End").clicked() {
            state.set(SimulationStateType::Finished);
            let mut game_state = cell.resource_mut::<NextState<GameState>>();
            game_state.set(GameState::Menu);
        }
    });

    let simulation_timer = cell.resource::<SimulationTimer>();
    ui.horizontal(|ui| {
        ui.add_sized(
            egui::Vec2::new(ui.available_width() * horizontal_ratio, 0.),
            egui::Label::new("Elapsed Time:"),
        );
        ui.add(egui::Label::new(
            simulation_timer.elapsed_secs_f64().round().to_string(),
        ));
    });

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
                match ui_state.generation_counter_target.parse::<u64>() {
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

    let mut step = cell.resource_mut::<SimulationStep>();
    ui.horizontal(|ui| {
        ui.add_sized(
            egui::Vec2::new(ui.available_width() * horizontal_ratio, 0.),
            egui::Label::new("Steps/s:"),
        );
        let generation_target_response = ui.add_sized(
            egui::Vec2::new(ui.available_width(), 0.),
            egui::TextEdit::singleline(&mut ui_state.steps_per_second),
        );
        if generation_target_response.lost_focus() {
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                match ui_state.steps_per_second.parse::<f64>() {
                    Ok(val) => {
                        step.steps_per_second = val;
                    }
                    Err(_) => {
                        ui_state.steps_per_second = format!("{:.2}", step.steps_per_second);
                    }
                }
            } else {
                ui_state.steps_per_second = format!("{:.2}", step.steps_per_second);
            }
        }
    });
}
