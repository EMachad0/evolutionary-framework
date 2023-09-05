use bevy::prelude::Reflect;
use bevy::utils::HashMap;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::simulation::population::genes::Chromosome;

#[derive(Debug, Clone, Reflect)]
pub struct Perm(Vec<i32>);

impl Chromosome for Perm {
    type I = ();
    type G = i32;

    fn new(dim: usize, _: &Self::I) -> Self {
        let mut gene = (0..dim as i32).collect::<Vec<i32>>();
        let mut rng = thread_rng();
        gene.shuffle(&mut rng);
        Self(gene)
    }

    fn get(&self) -> &Vec<Self::G> {
        &self.0
    }

    fn get_mut(&mut self) -> &mut Vec<Self::G> {
        &mut self.0
    }

    fn crossover(&mut self, other: &mut Self, prob: f64) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(1. - prob) {
            return;
        }

        let a = self.get_mut();
        let b = other.get_mut();
        let len = a.len();

        let point_1 = rng.gen_range(0..len);
        let point_2 = rng.gen_range(point_1..len);

        let mut a_mapping: HashMap<i32, i32> = HashMap::new();
        let mut b_mapping: HashMap<i32, i32> = HashMap::new();

        for i in point_1..=point_2 {
            a_mapping.insert(a[i], b[i]);
            b_mapping.insert(b[i], a[i]);
        }

        for i in 0..len {
            if i >= point_1 && i <= point_2 {
                continue;
            }

            let mut value_to_find = a[i];
            while let Some(&new_value) = a_mapping.get(&value_to_find) {
                value_to_find = new_value;
            }
            a[i] = value_to_find;

            value_to_find = b[i];
            while let Some(&new_value) = b_mapping.get(&value_to_find) {
                value_to_find = new_value;
            }
            b[i] = value_to_find;
        }
    }

    fn mutate(&mut self, prob: f64) {
        let perm = self.get_mut();
        let mut rng = thread_rng();
        for i in 0..perm.len() {
            if rng.gen_bool(prob) {
                let j = rng.gen_range(0..perm.len());
                perm.swap(i, j);
            }
        }
    }
}
