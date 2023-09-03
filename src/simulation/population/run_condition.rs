use bevy::prelude::*;

use crate::simulation::population::genes::{Gene, Chromosome};

pub fn population_type<G: Chromosome>(query: Query<&Gene<G>>) -> bool {
    !query.is_empty()
}
