use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::gui::{
    labels::Labelled,
    selection::{Clickable, CanFollow, Followed, Selected}
};
use crate::physics::PhysicsSettings;

pub struct BodyPlugin;

impl Plugin for BodyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(AmbientLight {
                color: Color::NONE,
                brightness: 0.0,
            })
            .insert_resource(Msaa::Sample8)
            .add_systems(Startup, spawn_bodies)
            .add_systems(First, add_materials);
    }
}

#[derive(Default, Clone)]
pub struct BodySetting {
    name: &'static str,
    velocity: Vec3,
    position: Vec3,
    mu: f32,
    radius: f32,
    material: StandardMaterial,
}

impl BodySetting {
    fn orbiting(mut self, orbiting: &Self, axis: Vec3) -> Self {
        let distance = self.position - orbiting.position;

        self.velocity = distance.cross(axis).normalize()
            * ((self.mu + orbiting.mu) / distance.length()).sqrt()
            + orbiting.velocity;

        self
    }
}

#[derive(Component, Clone)]
pub struct BodyMaterial {
    pub mesh: Mesh,
    pub material: StandardMaterial,
}

impl Default for BodyMaterial {
    fn default() -> Self {
        Self {
            mesh: shape::Cube { size: 10.0 }.into(),
            material: StandardMaterial::default(),
        }
    }
}

fn add_materials(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &BodyMaterial), Added<BodyMaterial>>,
) {
    for (entity, material) in &query {
        let mut cmds = commands.entity(entity);
        let BodyMaterial { mesh, material } = material.clone();

        cmds.insert((
            meshes.add(mesh),
            materials.add(material),
            VisibilityBundle::default(),
        ));
    }
}

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub friction: Friction,
    pub transform: TransformBundle,
    pub mass: ColliderMassProperties,
    pub read_mass: ReadMassProperties,
}

#[derive(Bundle, Default)]
pub struct BodyBundle {
    pub name: Name,
    pub labelled: Labelled,
    pub can_select: Clickable,
    pub can_follow: CanFollow,
    pub body_material: BodyMaterial,
    pub particle_bundle: ParticleBundle,
}


impl BodyBundle {
    pub fn new(setting: BodySetting) -> Self {
        Self {
            name: Name::new(setting.name),
            labelled: Labelled {
                style: TextStyle {
                    font_size: 6.0 * (1000.0 * setting.radius).log10(),
                    color: Color::GRAY,
                    ..default()
                },
                offset: Vec2::splat(setting.radius) * 1.1,
            },
            can_select: Clickable {
                radius: setting.radius,
            },
            can_follow: CanFollow {
                min_camera_distance: setting.radius * 3.0,
            },
            particle_bundle: ParticleBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::ball(setting.radius),
                velocity: Velocity::linear(setting.velocity),
                friction: Friction::coefficient(0.8),
                transform: TransformBundle::from(Transform::from_translation(setting.position)),
                mass: ColliderMassProperties::Mass(setting.mu),
                ..default()
            },
            body_material: BodyMaterial {
                mesh: shape::UVSphere {
                    radius: setting.radius,
                    ..default()
                }
                .into(),
                material: setting.material,
            },
        }
    }
}

pub fn spawn_bodies(
    mut commands: Commands,
    physics: Res<PhysicsSettings>,
) {
    let star_color = Color::rgb(1.0, 1.0, 0.9);
    let star = BodySetting {
        name: "Star",
        velocity: Vec3::new(-0.1826, -0.001, 0.0),
        mu: 5E3,
        radius: 8.0,
        material: StandardMaterial {
            base_color: star_color,
            emissive: star_color * 2.0,
            ..default()
        },
        ..default()
    };

    let planet = BodySetting {
        name: "Planet",
        position: Vec3::new(0.0, 60.0, 0.0),
        mu: 100.0,
        radius: 2.0,
        material: StandardMaterial {
            base_color: Color::rgb(0.0, 0.6, 1.0),
            ..default()
        },
        ..default()
    }
    .orbiting(&star, Vec3::Z);

    let moon = BodySetting {
        name: "Moon",
        position: planet.position + Vec3::new(4.5, 0.0, 0.0),
        mu: 1.0,
        radius: 0.6,
        material: StandardMaterial {
            base_color: Color::rgb(0.6, 0.4, 0.1),
            ..default()
        },
        ..default()
    }
    .orbiting(&planet, Vec3::new(0.0, 0.5, -1.0));

    let comet = BodySetting {
        name: "Comet",
        velocity: Vec3::new(2.8, 0.15, 0.4),
        position: Vec3::new(-200.0, 138.0, -18.0),
        mu: 0.000,
        radius: 0.1,
        material: StandardMaterial {
            base_color: Color::rgb(0.3, 0.3, 0.3),
            ..default()
        },
    };

    let star_bundle = BodyBundle::new(star);
    let star = commands.spawn((star_bundle, Selected)).id();

    let planet_bundle = BodyBundle::new(planet);
    commands.spawn(planet_bundle);

    let moon_bundle = BodyBundle::new(moon);
    commands.spawn(moon_bundle);

    let comet_bundle = BodyBundle::new(comet);
    commands.spawn(comet_bundle);

    commands.insert_resource(Followed(Some(star)));
}