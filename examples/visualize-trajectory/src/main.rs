use bevy::prelude::*;

mod bodies;
use bodies::{spawn_bodies, CelestialBodyPlugin};

mod gui;
use gui::GuiPlugin;

mod physics;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    canvas: Some("#app".to_owned()),
                    ..default()
                }),
                ..default()
            }),
            GuiPlugin,
            CelestialBodyPlugin,
            PhysicsPlugin,
        ))
        .add_systems(Startup, spawn_bodies)
        .run();
}
