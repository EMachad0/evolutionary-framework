use bevy::prelude::*;

use evolutionary_framework::simulation::population::fitness::Fitness;
use evolutionary_framework::simulation::population::genes::{Gene, GeneCod, Perm};

pub fn calc_fitness(mut individuals: Query<(&Gene<Perm>, &mut Fitness)>) {
    for (individual, mut fitness) in individuals.iter_mut() {
        let perm = individual.get().get();
        let n = perm.len();
        let mut diagonals1: Vec<i32> = vec![0; n * 2];
        let mut diagonals2: Vec<i32> = vec![0; n * 2];
        let mut value = 0;

        for (x, y) in perm.iter().enumerate() {
            let y = *y as usize;
            value += diagonals1[n + x - y];
            value += diagonals2[x + y];
            diagonals1[n + x - y] += 1;
            diagonals2[x + y] += 1;
        }

        let worst_case = n * (n - 1) / 2;
        fitness.set((worst_case as i32 - value) as f64 / worst_case as f64);
    }
}
