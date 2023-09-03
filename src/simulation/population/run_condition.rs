use bevy::prelude::*;

use crate::simulation::population::genes::{Gene, GeneCod};

pub fn population_type<G: GeneCod>(query: Query<&Gene<G>>) -> bool {
    !query.is_empty()
}
