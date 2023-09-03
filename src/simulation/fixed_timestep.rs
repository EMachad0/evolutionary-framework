use crate::simulation::SelectionSchedule;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::time::Duration;

use crate::GameState;

use crate::simulation::simulation_state::is_simulation_paused;

pub struct FixedTimestepPlugin;

impl Plugin for FixedTimestepPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FixedTimestep>()
            .add_systems(OnEnter(GameState::Playing), setup_simulation_fixed_timestep)
            .add_systems(PostUpdate, update_timer.run_if(not(is_simulation_paused)))
            .add_systems(Update, run_fixed_timestep);
    }
}

#[derive(Debug, Default, Component, Reflect)]
pub struct FixedTimestep {
    step: Duration,
    accumulator: Duration,
    #[reflect(ignore)]
    schedules: Vec<Box<dyn ScheduleLabel>>,
}

impl FixedTimestep {
    pub fn new(step: Duration) -> Self {
        Self { step, ..default() }
    }

    pub fn with_schedule<T: ScheduleLabel>(mut self, label: T) -> Self {
        self.schedules.push(Box::new(label));
        self
    }

    pub fn set_step(&mut self, step: Duration) {
        self.step = step
    }
}

pub fn setup_simulation_fixed_timestep(mut commands: Commands) {
    let step = Duration::from_secs_f64(1. / 60.);
    let fixed_timestep = FixedTimestep::new(step).with_schedule(SelectionSchedule);
    commands.spawn((Name::new("Simulation Fixed Timestep"), fixed_timestep));
}

pub fn update_timer(mut timers: Query<&mut FixedTimestep>, time: Res<Time>) {
    for mut timer in timers.iter_mut() {
        timer.accumulator += time.delta();
    }
}

pub fn run_fixed_timestep(world: &mut World) {
    let mut mapping: HashMap<Box<dyn ScheduleLabel>, u32> = HashMap::new();
    let mut timers = world.query::<&mut FixedTimestep>();
    for mut timer in timers.iter_mut(world) {
        let step = timer.step;
        let mut cnt = 0;
        while timer.accumulator >= step {
            cnt += 1;
            timer.accumulator -= step;
        }
        if cnt > 0 {
            for schedule in timer.schedules.iter() {
                mapping.insert(schedule.to_owned(), cnt);
            }
        }
    }

    for (schedule, cnt) in mapping.into_iter() {
        world.schedule_scope(schedule, |w, s| {
            for _ in 0..cnt {
                s.run(w);
            }
        });
    }
}
