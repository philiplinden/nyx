use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod bodies;
mod camera;
mod gui;
mod physics;

pub const DT: f32 = 1.0 / 60.0;

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

            // Interface
            gui::GuiPlugin,
            camera::CameraPlugin,

            //Physics
            RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false),
            physics::CustomRapierSchedule,
            bodies::BodyPlugin,
        ))
        .run();
}