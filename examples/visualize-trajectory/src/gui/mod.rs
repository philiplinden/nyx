use bevy::prelude::*;

mod camera;
pub mod controls;
pub mod selection;

use camera::{setup_camera, CameraPlugin};
use controls::ControlsPlugin;
use selection::SelectionPlugin;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, ControlsPlugin, SelectionPlugin))
            .add_systems(Startup, setup_camera);
    }
}

pub fn format_duration(duration: std::time::Duration, precision: usize) -> String {
    humantime::format_duration(duration)
        .to_string()
        .split_inclusive(' ')
        .take(precision)
        .collect::<String>()
}
