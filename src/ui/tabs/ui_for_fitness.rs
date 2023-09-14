use bevy::math::DVec2;
use bevy::prelude::*;
use egui::plot::{Line, Plot, PlotPoints};
use itertools::Itertools;

use crate::simulation::fitness_diagnostics::FitnessDiagnostics;

const MAX_PLOT_POINTS: usize = 100;

pub fn ui_for_fitness(world: &mut World, ui: &mut egui::Ui) {
    let history = world.resource::<FitnessDiagnostics>();

    let best_line = {
        let points = history
            .best
            .iter()
            .enumerate()
            .map(|(i, y)| DVec2::new(i as f64, *y))
            .collect_vec();
        let plot_points = points_to_ui(&points);
        Line::new(plot_points)
    };
    let avg_line = {
        let points = history
            .avg
            .iter()
            .enumerate()
            .map(|(i, y)| DVec2::new(i as f64, *y))
            .collect_vec();
        let plot_points = points_to_ui(&points);
        Line::new(plot_points)
    };

    Plot::new("Fitness").show(ui, |plot_ui| {
        plot_ui.line(best_line);
        plot_ui.line(avg_line);
    });
}

fn points_to_ui(points: &Vec<DVec2>) -> PlotPoints {
    let chunk_size = (points.len() / MAX_PLOT_POINTS).max(1);

    let over_time: PlotPoints = points
        .chunks(chunk_size)
        .map(|c| c.iter().sum::<DVec2>() / c.len() as f64)
        .map(|DVec2 { x, y }| [x, y])
        .collect();

    over_time
}
