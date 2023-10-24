use bevy::prelude::*;
use std::fmt;

use crate::maze::Maze;
use evolutionary_framework::simulation::population::genes::{Gene, Real};
use evolutionary_framework::simulation::population::objective::{
    objective_setup, update_name_from_objective, Objective,
};
use evolutionary_framework::simulation::{SimulationSchedule, SimulationSet};
use evolutionary_framework::GameState;

pub struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            objective_setup::<MazeObjective>.after(SimulationSet::PopulationStart),
        )
        .add_systems(
            SimulationSchedule,
            calc_objective.in_set(SimulationSet::Objective),
        )
        .add_systems(PostUpdate, update_name_from_objective::<MazeObjective>);
    }
}

#[derive(Debug, Default, Copy, Clone, Component, Reflect)]
pub struct MazeObjective {
    pub position: (usize, usize),
    pub distance: usize,
}

impl Objective for MazeObjective {}

impl fmt::Display for MazeObjective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (x, y) = self.position;
        f.write_fmt(format_args!("x={} y={}", x, y))
    }
}

const DIR: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

pub fn calc_objective(mut query: Query<(&mut MazeObjective, &Gene<Real>)>, maze: Res<Maze>) {
    query.par_iter_mut().for_each_mut(|(mut objective, gene)| {
        let mut path = vec![maze.begin];
        for g in gene.get() {
            let mut possibilities = Vec::new();
            let (x, y) = *path.last().unwrap();
            for (dx, dy) in DIR {
                let oxx = x.checked_add_signed(dx);
                let oyy = y.checked_add_signed(dy);
                if let (Some(xx), Some(yy)) = (oxx, oyy) {
                    if let Some(v) = maze.get(xx, yy) {
                        if v != 0 && !path.contains(&(xx, yy)) {
                            possibilities.push((xx, yy));
                        }
                    }
                }
            }
            if possibilities.is_empty() {
                break;
            }
            let idx = (possibilities.len() as f64 * g).floor() as usize;
            let next_position = possibilities[idx];
            path.push(next_position);
        }

        let (tx, ty) = maze.end;

        let best = path
            .iter()
            // .min_by_key(|(x, y)| euclidean_distance(*x, *y, tx, ty))
            .last()
            .unwrap();

        let dist = euclidean_distance(best.0, best.1, tx, ty);
        *objective = MazeObjective {
            position: *best,
            distance: dist,
        };
    });
}

pub fn euclidean_distance(x0: usize, y0: usize, x1: usize, y1: usize) -> usize {
    x0.max(x1) - x0.min(x1) + y0.max(y1) - y0.min(y1)
}
