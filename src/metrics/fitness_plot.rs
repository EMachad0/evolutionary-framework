use bevy::prelude::*;
use plotters::prelude::*;
use plotters::style::Color;

use crate::metrics::fitness_history::RunsFitnessHistory;
use crate::metrics::is_auto_runner;
use crate::metrics::run_counter::counter_just_finished;
use crate::GameState;

pub struct FitnessPlotPlugin;

impl Plugin for FitnessPlotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            plot_chart.run_if(is_auto_runner.and_then(counter_just_finished)),
        );
    }
}

fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

fn variance(data: &[f64]) -> f64 {
    let avg = mean(data);
    data.iter().map(|&x| (x - avg).powi(2)).sum::<f64>() / (data.len() - 1) as f64
}

fn standard_deviation(data: &[f64]) -> f64 {
    variance(data).sqrt()
}

fn confidence_interval(data: &[f64], confidence: f64) -> (f64, f64) {
    let avg = mean(data);
    let stderr = standard_deviation(data) / (data.len() as f64).sqrt();
    let z_value = (1.0 - confidence) / 2.0;
    let margin_error = z_value * stderr;
    (avg - margin_error, avg + margin_error)
}

fn calculate_confidence_intervals_and_means(
    history: &RunsFitnessHistory,
    confidence: f64,
) -> (Vec<f64>, Vec<(f64, f64)>, Vec<f64>, Vec<(f64, f64)>) {
    let num_points = history[0].avg.len();

    let mut avg_means = Vec::with_capacity(num_points);
    let mut avg_intervals = Vec::with_capacity(num_points);
    let mut best_means = Vec::with_capacity(num_points);
    let mut best_intervals = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let avg_values: Vec<f64> = history.iter().map(|diag| diag.avg[i]).collect();
        avg_means.push(avg_values.iter().sum::<f64>() / avg_values.len() as f64);
        avg_intervals.push(confidence_interval(&avg_values, confidence));

        let best_values: Vec<f64> = history.iter().map(|diag| diag.best[i]).collect();
        best_means.push(best_values.iter().sum::<f64>() / best_values.len() as f64);
        best_intervals.push(confidence_interval(&best_values, confidence));
    }

    (avg_means, avg_intervals, best_means, best_intervals)
}

pub fn plot_chart(runs: Res<RunsFitnessHistory>) {
    let (avg_means, avg_intervals, best_means, best_intervals) =
        calculate_confidence_intervals_and_means(&runs, 0.95);

    let root = BitMapBackend::new("simulation.png", (1366, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Fitness Over Generations with Confidence Bands",
            ("sans-serif", 20),
        )
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..avg_intervals.len() as f64, 0f64..1.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let line_thickness: u32 = 3;

    chart
        .draw_series(LineSeries::new(
            avg_means.iter().enumerate().map(|(x, &y)| (x as f64, y)),
            RED.stroke_width(line_thickness),
        ))
        .unwrap()
        .label("Avg")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(LineSeries::new(
            best_means.iter().enumerate().map(|(x, &y)| (x as f64, y)),
            BLUE.stroke_width(line_thickness),
        ))
        .unwrap()
        .label("Best")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    let avg_polygon: Vec<_> = avg_intervals
        .iter()
        .enumerate()
        .map(|(x, &(_, u))| (x as f64, u))
        .chain(
            avg_intervals
                .iter()
                .enumerate()
                .rev()
                .map(|(x, &(l, _))| (x as f64, l)),
        )
        .collect();

    chart
        .draw_series(AreaSeries::new(
            avg_polygon.clone(),
            0.0, // Start of Y coordinate for the filled area (this assumes the baseline is at y = 0)
            RGBAColor(255, 0, 0, 0.1),
        ))
        .unwrap();

    let best_polygon: Vec<_> = best_intervals
        .iter()
        .enumerate()
        .map(|(x, &(_, u))| (x as f64, u))
        .chain(
            best_intervals
                .iter()
                .enumerate()
                .rev()
                .map(|(x, &(l, _))| (x as f64, l)),
        )
        .collect();

    chart
        .draw_series(AreaSeries::new(
            best_polygon.clone(),
            0.0, // Start of Y coordinate for the filled area (this assumes the baseline is at y = 0)
            RGBAColor(0, 0, 255, 0.1),
        ))
        .unwrap();

    chart.configure_series_labels().draw().unwrap();
}
