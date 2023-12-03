#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

pub mod model;
use crate::model::{
    kinematics::*,
    nbody::*,
    orbit_prediction::*,
};
pub mod visualizer;
use crate::visualizer::{
    camera::*,
    selection::*,
    ui::*,
};

/**
 * For a Bevy app, the main function is the hub where all plugins, resources,
 * and systems are added so the engine is aware of them.
 */
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    #[cfg(not(target_arch = "wasm32"))]
                    resolution: bevy::window::WindowResolution::new(1920.0, 1080.0),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    canvas: Some("#app".to_owned()),
                    ..default()
                }),
                ..default()
            }),
            CameraPlugin,
            SelectionPlugin,
            UiPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::NONE,
            brightness: 0.0,
        })
        .insert_resource(PhysicsSettings::delta_time(1.0 / 60.0))
        .add_systems(Startup, setup_scene)
        .add_systems(First, add_materials)
        .run();
}

/**
 * Spawn entities with their models and set the initial conditions.
 * The camera is included in the entities spawned here.
 */
fn setup_scene(
    mut commands: Commands,
    mut event_writer: EventWriter<ComputePredictionEvent>,
    kinematics: Res<PhysicsSettings>,
) {
    /*!
     * Set up the scene space with a camera in an appropriate location to see
     * the rendered objects. A sensible start position is on the ecliptic and
     * pointed toward the origin of the reference frame.
     *
     * NOTE: At the end of the setup function, the camera is commanded to follow
     *       one of the spawned bodies. Its relative position to the body it is 
     *       following is determined by this initial position in absolute space.
     */
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 200.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        OrbitCamera::default(),
        bevy::core_pipeline::bloom::BloomSettings {
            intensity: 0.15,
            ..default()
        },
    ));

    // Describe the Sun
    let sun_color = Color::rgb(1.0, 1.0, 0.9);
    // TODO: Replace nbody with Nyx body
    let sun = BodySetting {
        name: "Sun",
        velocity: Vec3::new(-0.1826, -0.001, 0.0),
        mu: 5E3,
        radius: 8.0,
        material: StandardMaterial {
            base_color: sun_color,
            emissive: sun_color * 2.0,
            ..default()
        },
        ..default()
    };

    // Describe the Earth at the origin. Represent it with a low-poly model.
    let earth = BodySetting {
        name: "Earth",
        position: Vec3::new(0.0, 60.0, 0.0),
        mu: 100.0,
        radius: 2.0,
        material: StandardMaterial {
            base_color: Color::rgb(0.0, 0.6, 1.0),
            ..default()
        },
        ..default()
    }
    .orbiting(&sun, Vec3::Z);

    // Describe Luna. Represent it with a low-poly model.
    let luna = BodySetting {
        name: "Luna",
        position: earth.position + Vec3::new(4.5, 0.0, 0.0),
        mu: 1.0,
        radius: 0.6,
        material: StandardMaterial {
            base_color: Color::rgb(0.6, 0.4, 0.1),
            ..default()
        },
        ..default()
    }
    .orbiting(&earth, Vec3::new(0.0, 0.5, -1.0));

    // Spawn the sun.
    let mut sun_bundle = BodyBundle::new(sun);
    sun_bundle.prediction_bundle.draw.steps = Some(0);
    let sun = commands.spawn((sun_bundle, Selected)).id();

    // Spawn Earth
    let mut earth_bundle = BodyBundle::new(earth);
    earth_bundle.prediction_bundle.draw.reference = Some(sun);
    let earth = commands.spawn(earth_bundle).id();

    // Spawn Luna
    let mut luna_bundle = BodyBundle::new(luna);
    luna_bundle.prediction_bundle.draw.reference = Some(earth);
    commands.spawn(luna_bundle);

    // Snap the camera to a body.
    commands.insert_resource(Followed(Some(earth)));

    // Tell the simulation to start ticking.
    event_writer.send(ComputePredictionEvent {
        steps: kinematics.steps_per_second() * 60 * 5,
    });
}

/**
 * Define some simple materials for the renderer to use. This is only necessary
 * for primitive geometries, as the .glb assets should already have materials
 * baked in. If something gets borked or forgotton, use the default material
 * from Bevy and a simple cube mesh.
 */
#[derive(Component, Clone)]
pub struct BodyMaterial {
    pub mesh: Mesh,
    pub material: StandardMaterial,
}

impl Default for BodyMaterial {
    fn default() -> Self {
        Self {
            // All hail the default cube!
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
        if material.emissive != Color::BLACK {
            cmds.with_children(|child| {
                child.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: material.emissive,
                        intensity: 5E4,
                        range: 2E3,
                        shadows_enabled: true,
                        ..default()
                    },
                    // Put the light at the center of the body
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                });
            });
        }

        // PBR == Physically Based Rendering. Color, reflectance, normals, etc
        cmds.insert(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(material),
            ..default()
        });
    }
}

/**
 * A bundle for Particular particles.
 * 
 * TODO: Replace this with a Nyx body interface.
 */
#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub interolated: Interpolated,
    pub acceleration: Acceleration,
    pub velocity: Velocity,
    pub position: Position,
    pub mass: Mass,
}

/**
 * A bundle that associates physics and UI elements with a body.
 * 
 * TODO: Replace the ParticleBundle and PredictionBundle with Nyx components.
 */
#[derive(Bundle, Default)]
pub struct BodyBundle {
    pub name: Name,
    pub labelled: Labelled,
    pub can_select: CanSelect,
    pub can_follow: CanFollow,
    pub body_material: BodyMaterial,
    pub particle_bundle: ParticleBundle,
    pub prediction_bundle: PredictionBundle,
}

/**
 * This struct holds the set of parameters needed to initialize a body in the
 * simulation. We can set up one entity orbiting another with the `orbiting`
 * method. This method is basically a shortcut for assigning the body a velocity
 * that would result in a circular orbit around the other referenced body.
 * 
 * TODO: adapt this for the Nyx interface.
 */
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
            can_select: CanSelect {
                radius: setting.radius,
            },
            // Transform the camera with the body as it translates.
            can_follow: CanFollow {
                min_camera_distance: setting.radius * 3.0,
                saved_transform: Transform::from_xyz(0.0, 0.0, setting.radius * 20.0),
            },
            particle_bundle: ParticleBundle {
                mass: Mass(setting.mu),
                velocity: Velocity(setting.velocity),
                position: Position(setting.position),
                ..default()
            },
            // This bundle displays the trajectory.
            prediction_bundle: PredictionBundle {
                draw: PredictionDraw {
                    color: setting.material.base_color,
                    ..default()
                },
                ..default()
            },
            // TODO: Replace this with fancy assets.
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

pub fn format_duration(duration: std::time::Duration, precision: usize) -> String {
    humantime::format_duration(duration)
        .to_string()
        .split_inclusive(' ')
        .take(precision)
        .collect::<String>()
}
