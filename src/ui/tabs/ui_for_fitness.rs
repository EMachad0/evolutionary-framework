use bevy::prelude::*;
use egui::plot::{Line, Plot, PlotPoints};

use crate::simulation::fitness_diagnostics::FitnessHistory;

pub fn ui_for_fitness(world: &mut World, ui: &mut egui::Ui) {
    let history = world.resource::<FitnessHistory>();

    let best_line = {
        let points = history.best.iter().enumerate().collect::<Vec<_>>();
        let over_time: PlotPoints = points.into_iter().map(|(i, v)| [i as f64, *v]).collect();
        Line::new(over_time)
    };
    let avg_line = {
        let points = history.avg.iter().enumerate().collect::<Vec<_>>();
        let over_time: PlotPoints = points.into_iter().map(|(i, v)| [i as f64, *v]).collect();
        Line::new(over_time)
    };

    Plot::new("Fitness").show(ui, |plot_ui| {
        plot_ui.line(best_line);
        plot_ui.line(avg_line);
    });
}
