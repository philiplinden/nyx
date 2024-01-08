use bevy::prelude::*;

mod bodies;
use bodies::BodyPlugin;

mod gui;
use gui::GuiPlugin;

mod physics;
use physics::{PhysicsPlugin, PhysicsSettings};

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
            BodyPlugin,
            PhysicsPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::NONE,
            brightness: 0.0,
        })
        .run();
}
