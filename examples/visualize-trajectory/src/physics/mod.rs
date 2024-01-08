use std::time::Duration;

use bevy::prelude::*;

pub mod nbody;
mod schedule;

pub use schedule::CustomRapierSchedule;

#[derive(Resource, Clone, Copy)]
pub struct PhysicsSettings {
    pub delta_time: f32,
    pub time_scale: f32,
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            delta_time: 1.0 / 60.0,
            time_scale: 1.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct PhysicsTime {
    accumulated: f32,
    pub paused: bool,
}

impl PhysicsTime {
    fn tick(&mut self, delta: f32) {
        if !self.paused {
            self.accumulated += delta;
        }
    }

    fn can_step(&self, period: f32) -> bool {
        !self.paused && self.accumulated >= period
    }
}

#[derive(Resource, Deref, Clone, Copy, Default)]
pub struct ElapsedPhysicsTime(Duration);

#[derive(Component, Clone, Copy, Default, Deref, DerefMut)]
pub struct Mass(pub f32);
