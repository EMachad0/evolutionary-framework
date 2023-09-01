use bevy::prelude::*;

use crate::simulation::population::genes::{Gene, GeneCod};

pub fn population_type<G>(query: Query<&Gene<G>>) -> bool
where
    G: 'static + Send + Sync + GeneCod,
{
    !query.is_empty()
}
