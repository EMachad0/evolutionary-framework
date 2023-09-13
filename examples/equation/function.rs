use bevy::prelude::*;

use evolutionary_framework::GameState;

fn objetive_function(x: f32) -> f32 {
    (20. * x).cos() - (x.abs() / 2.) + (x * x * x / 4.)
}

#[derive(Debug, Copy, Clone, Resource)]
pub struct Function {
    pub f: fn(f32) -> f32,
    pub x_domain: (f32, f32),
    pub y_domain: (f32, f32),
}

pub struct FunctionPlugin;

impl Plugin for FunctionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init_function);
    }
}

pub fn init_function(world: &mut World) {
    let x_domain = (-2., 2.);
    let y_domain = (-4., 2.);
    world.insert_resource(Function {
        f: objetive_function,
        x_domain,
        y_domain,
    })
}
