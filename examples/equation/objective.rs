use bevy::prelude::*;
use std::fmt;

use crate::function::Function;
use evolutionary_framework::simulation::population::genes::{Bool, Gene};
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
            objective_setup::<EquationObjective>.after(SimulationSet::PopulationStart),
        )
        .add_systems(
            SimulationSchedule,
            calc_objective.in_set(SimulationSet::Objective),
        )
        .add_systems(PostUpdate, update_name_from_objective::<EquationObjective>);
    }
}

#[derive(Debug, Default, Copy, Clone, Component, Reflect)]
pub struct EquationObjective {
    pub x: f32,
    pub y: f32,
}

impl Objective for EquationObjective {}

impl fmt::Display for EquationObjective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("x={:.2} y={:.2}", self.x, self.y))
    }
}

pub fn calc_objective(
    mut query: Query<(&mut EquationObjective, &Gene<Bool>)>,
    function: Query<&Function>,
) {
    let function = function.single();
    let x_domain = function.x_domain;
    for (mut objective, gene) in query.iter_mut() {
        let x = gene_to_value(gene) * (x_domain.1 - x_domain.0) + x_domain.0;
        let y = (function.f)(x);
        *objective = EquationObjective { x, y };
    }
}

pub fn gene_to_value(gene: &Gene<Bool>) -> f32 {
    let gene = gene.get();
    let mut dec = 0.;
    for b in gene.iter().rev() {
        if *b {
            dec += 1.;
        }
        dec /= 2.;
    }
    dec
}
