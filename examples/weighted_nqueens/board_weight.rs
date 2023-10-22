use bevy::prelude::*;
use ordered_float::OrderedFloat;

use evolutionary_framework::config::{Config, ConfigSet};
use evolutionary_framework::GameState;

pub struct BoardWeightPlugin;

impl Plugin for BoardWeightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardWeight>().add_systems(
            OnExit(GameState::Loading),
            init_board_weights.after(ConfigSet),
        );
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct BoardWeight {
    pub board_size: usize,
    pub max_score: f64,
    weights: Vec<f64>,
}

impl std::ops::Index<usize> for BoardWeight {
    type Output = [f64];

    fn index(&self, index: usize) -> &Self::Output {
        self.weights.chunks(self.board_size).nth(index).unwrap()
    }
}

fn init_board_weights(mut board: ResMut<BoardWeight>, config: Res<Config>) {
    let n = config.population.dim;
    let mut weights = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            let v = i * n + j + 1;
            let v = if i % 2 == 0 {
                (v as f64).sqrt()
            } else {
                (v as f64).log10()
            };
            weights.push(v);
        }
    }

    let mut sorted_weights = weights.clone();
    sorted_weights.sort_by_key(|f| OrderedFloat(*f));
    let max_score = sorted_weights.iter().rev().take(n).sum::<f64>();

    *board = BoardWeight {
        board_size: n,
        weights,
        max_score,
    };
}
