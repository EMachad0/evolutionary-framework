use bevy::prelude::*;
use evolutionary_framework::GameState;

use evolutionary_framework::simulation::population::fitness::Fitness;
use evolutionary_framework::simulation::population::genes::{Bool, Gene};

pub struct FitnessPlugin;

impl Plugin for FitnessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            calc_fitness.run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn calc_fitness(mut individuals: Query<(&Gene<Bool>, &mut Fitness)>) {
    for (gene, mut fitness) in individuals.iter_mut() {
        let (st, lx) = gene_to_value(gene);

        let fit = 30 * st + 40 * lx;
        let fit = fit as f64 / 1360.;
        let hn = ((st + 2 * lx - 40) as f64 / 16.).max(0.);

        fitness.set(fit - hn);
    }
}

pub fn gene_to_value(gene: &Gene<Bool>) -> (i32, i32) {
    let gene = gene.get();
    let mut st = 0;
    for bit in gene.iter().take(5) {
        st *= 2;
        if *bit {
            st += 1;
        }
    }
    let mut lx = 0;
    for bit in gene.iter().skip(5) {
        lx *= 2;
        if *bit {
            lx += 1;
        }
    }
    let stn = st as f64 / 31. * 24.;
    let lxn = lx as f64 / 31. * 16.;
    (stn.floor() as i32, lxn.floor() as i32)
}
