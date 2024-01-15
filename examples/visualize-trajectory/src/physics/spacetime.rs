use bevy::prelude::*;
use particular::prelude::*;

use crate::physics::schedule::{PhysicsSchedule, PhysicsSet};

// from particular::prelude
pub const NBODY_COMPUTE_METHOD: sequential::BruteForcePairs = sequential::BruteForcePairs;

#[derive(Component, Clone, Copy, Default, Deref, DerefMut, Reflect)]
pub struct Position(pub Vec3);

#[derive(Component, Clone, Copy, Default, Deref, DerefMut, Reflect)]
pub struct Velocity(pub Vec3);

#[derive(Component, Clone, Copy, Default, Deref, DerefMut, Reflect)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Clone, Copy, Default, Deref, DerefMut)]
pub struct Mass(pub f32);

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PhysicsSchedule,
            accelerate_particles.in_set(PhysicsSet::First),
        );
    }
}

pub fn sympletic_euler(
    acceleration: Vec3,
    mut velocity: Vec3,
    mut position: Vec3,
    dt: f32,
) -> (Vec3, Vec3) {
    velocity += acceleration * dt;
    position += velocity * dt;

    (velocity, position)
}

fn accelerate_particles(mut query: Query<(&mut Acceleration, &Position, &Mass)>) {
    query
        .iter()
        .map(|(.., position, mass)| (position.to_array(), **mass))
        .accelerations(&mut NBODY_COMPUTE_METHOD.clone())
        .map(Vec3::from)
        .zip(&mut query)
        .for_each(|(acceleration, (mut physics_acceleration, ..))| {
            **physics_acceleration = acceleration
        });
}