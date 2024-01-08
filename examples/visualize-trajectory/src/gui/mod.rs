use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod controls;
pub mod labels;
pub mod selection;

use self::{labels::LabelsPlugin, selection::SelectionPlugin};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin,
            SelectionPlugin,
            LabelsPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        ))
        .add_systems(PostStartup, setup_egui)
        .add_systems(
            Update,
            (controls::selection_window, controls::simulation_window),
        );
    }
}

fn setup_egui(mut ctxs: EguiContexts) {
    ctxs.ctx_mut().set_visuals(egui::Visuals {
        window_fill: egui::Color32::from_rgba_premultiplied(27, 27, 27, 225),
        window_stroke: egui::Stroke::NONE,
        ..egui::Visuals::dark()
    });
}

pub fn format_duration(duration: std::time::Duration, precision: usize) -> String {
    humantime::format_duration(duration)
        .to_string()
        .split_inclusive(' ')
        .take(precision)
        .collect::<String>()
}
