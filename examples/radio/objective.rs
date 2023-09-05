use bevy::prelude::*;
use std::fmt;

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
            objective_setup::<RadioObjective>.after(SimulationSet::PopulationStart),
        )
        .add_systems(
            SimulationSchedule,
            calc_objective.in_set(SimulationSet::Objective),
        )
        .add_systems(PostUpdate, update_name_from_objective::<RadioObjective>);
    }
}

#[derive(Debug, Default, Copy, Clone, Component, Reflect)]
pub struct RadioObjective {
    pub st: i32,
    pub lx: i32,
}

impl RadioObjective {
    pub fn gain(&self) -> i32 {
        self.st * 30 + self.lx * 40
    }

    pub fn restriction(&self) -> i32 {
        self.st + 2 * self.lx - 40
    }
}

impl Objective for RadioObjective {}

impl fmt::Display for RadioObjective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "st={} lx={} g={} r={}",
            self.st,
            self.lx,
            self.gain(),
            self.restriction(),
        ))
    }
}

pub fn calc_objective(mut query: Query<(&mut RadioObjective, &Gene<Bool>)>) {
    for (mut objective, gene) in query.iter_mut() {
        let (st, lx) = gene_to_value(gene);
        *objective = RadioObjective { st, lx };
    }
}

pub fn gene_to_value(gene: &Gene<Bool>) -> (i32, i32) {
    let gene = gene.get();
    let mut st = 0;
    for bit in gene.iter().take(5).rev() {
        st *= 2;
        if *bit {
            st += 1;
        }
    }
    let mut lx = 0;
    for bit in gene.iter().skip(5).rev() {
        lx *= 2;
        if *bit {
            lx += 1;
        }
    }
    let stn = st as f64 / 31. * 24.;
    let lxn = lx as f64 / 31. * 16.;
    (stn.round() as i32, lxn.round() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use evolutionary_framework::simulation::population::genes::{Bool, Gene};
    use itertools::Itertools;

    #[test]
    pub fn it_returns_st_lx() {
        let cromo_str = "1011011111".to_string();
        let cromo = cromo_str.chars().map(|c| c == '1').collect_vec();
        let gene = Gene(Bool(cromo));

        let (st, lx) = gene_to_value(&gene);
        assert_eq!(st, 10);
        assert_eq!(lx, 16);
    }
}
