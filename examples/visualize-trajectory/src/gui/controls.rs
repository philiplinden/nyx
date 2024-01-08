use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::gui::{
    selection::{Selected, Followed},
    format_duration,
};

use crate::physics::{
    PhysicsSettings, PhysicsTime, ElapsedPhysicsTime,
};

trait DurationSlider<'a> {
    fn new_duration<Num: egui::emath::Numeric>(
        value: &'a mut Num,
        range: std::ops::RangeInclusive<Num>,
        delta: f32,
        precision: usize,
    ) -> egui::Slider<'a> {
        egui::Slider::new(value, range).custom_formatter(move |s, _| {
            format_duration(Duration::from_secs_f32(s as f32 * delta), precision)
        })
    }
}

impl DurationSlider<'_> for egui::Slider<'_> {}



pub fn simulation_window(
    mut ctxs: EguiContexts,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
    elapsed_time: Res<ElapsedPhysicsTime>,
    mut physics: ResMut<PhysicsSettings>,
    mut physics_time: ResMut<PhysicsTime>,
) {
    egui::Window::new("Simulation settings")
        .default_width(255.0)
        .resizable(false)
        .anchor(egui::Align2::RIGHT_TOP, [0.0, 0.0])
        .show(ctxs.ctx_mut(), |ui| {
            if let Some(fps) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    ui.horizontal(|ui| {
                        ui.label("FPS:");
                        ui.label(format!("{value:.2}"));
                    });
                }
            }

            ui.horizontal(|ui| {
                ui.label("Time elapsed:");
                ui.label(format_duration(**elapsed_time, 3));
            });

            ui.horizontal(|ui| {
                ui.label("Time scale:");
                ui.add(egui::Slider::new(&mut physics.time_scale, 0.05..=100.0).logarithmic(true));
            });

            ui.checkbox(&mut physics_time.paused, "Paused");
        });
}

pub fn selection_window(
    mut ctxs: EguiContexts,
    mut followed: ResMut<Followed>,
    query_selection: Query<(Option<Entity>, &Name, bevy::ecs::query::Has<Selected>)>,
) {
    for (entity, selected_name, is_selected) in &query_selection {
        if !is_selected {
            continue;
        }

        egui::Window::new(selected_name.to_string())
            .default_width(245.0)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
            .show(ctxs.ctx_mut(), |ui| {
                ui.heading("Camera");

                if ui.button("Follow").clicked() {
                    **followed = entity;
                }
            });
    }
}