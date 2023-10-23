use anyhow::Context;
use bevy::prelude::*;
use itertools::Itertools;

use crate::config::ConfigAssets;
use evolutionary_framework::assets::txt_asset::TxtAsset;
use evolutionary_framework::config::{ConfigSet, CONFIG_SCHEDULE};

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Maze>()
            .init_resource::<Maze>()
            .add_systems(CONFIG_SCHEDULE, maze_from_asset.after(ConfigSet));
    }
}

pub fn maze_from_asset(
    handles: Res<ConfigAssets>,
    assets: Res<Assets<TxtAsset>>,
    mut maze: ResMut<Maze>,
) {
    let handle = &handles.maze;
    let maze_str = assets.get(handle).unwrap();
    *maze = Maze::from_str(maze_str);
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub begin: (usize, usize),
    pub end: (usize, usize),
    pub data: Vec<i8>,
}

impl Maze {
    pub fn from_str(data_str: &str) -> Self {
        let data = data_str
            .split('\n')
            .map(|row| {
                let row = row.trim();
                row[1..row.len() - 1]
                    .split(',')
                    .map(|v| {
                        v.parse::<i8>()
                            .context(format!("Unable to parse i8 from {:?}", v))
                            .unwrap()
                    })
                    .collect_vec()
            })
            .collect_vec();

        let width = data[0].len();
        let height = data.len();

        let mut begin = None;
        let mut end = None;
        for i in 0..height {
            for j in 0..width {
                match data[i][j] {
                    2 => begin = Some((j, i)),
                    3 => end = Some((j, i)),
                    _ => {}
                };
            }
        }

        Maze {
            width,
            height,
            begin: begin.expect("Unable to find begin position"),
            end: end.expect("Unable to find end position"),
            data: data.into_iter().flatten().collect_vec(),
        }
    }
}

impl std::ops::Index<usize> for Maze {
    type Output = [i8];

    fn index(&self, index: usize) -> &Self::Output {
        let l = self.width * index;
        let r = l + self.width;
        &self.data[l..r]
    }
}
