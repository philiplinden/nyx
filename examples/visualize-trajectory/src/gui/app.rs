use eframe::{App, CreationContext};

use crate::gui::controls::ScenarioPicker;
use crate::gui::View;

pub struct NyxGui {
    scenario_picker: ScenarioPicker,
}

impl Default for NyxGui {
    fn default() -> Self {
        Self {
            // specify defaults here
            scenario_picker: ScenarioPicker::default(),
        }
    }
}

impl NyxGui {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl App for NyxGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // egui::SidePanel::left("sidebar")
        //     .resizable(true)
        //     .default_width(150.0)
        //     .show(ctx, |ui| {egui::menu::bar(ui, |ui| {
        //             file_menu_button(ui);
        //         });
        //     });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            // add the dropdown to select a reference frame
            self.scenario_picker.ui(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            todo!("Add the plots")
        });
    }
}

fn file_menu_button(ui: &mut egui::Ui) {
    let organize_shortcut =
        egui::KeyboardShortcut::new(egui::Modifiers::CTRL | egui::Modifiers::SHIFT, egui::Key::O);
    let reset_shortcut =
        egui::KeyboardShortcut::new(egui::Modifiers::CTRL | egui::Modifiers::SHIFT, egui::Key::R);

    // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
    // or else they would only be checked if the "File" menu was actually open!

    if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
    }

    if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
    }

    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        // On the web the browser controls the zoom
        #[cfg(not(target_arch = "wasm32"))]
        {
            egui::gui_zoom::zoom_menu_buttons(ui);
            ui.weak(format!(
                "Current zoom: {:.0}%",
                100.0 * ui.ctx().zoom_factor()
            ))
            .on_hover_text("The UI zoom level, on top of the operating system's default value");
            ui.separator();
        }

        if ui
            .add(
                egui::Button::new("Organize Windows")
                    .shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
            )
            .clicked()
        {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
            ui.close_menu();
        }

        if ui
            .add(
                egui::Button::new("Reset egui memory")
                    .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
            )
            .on_hover_text("Forget scroll, positions, sizes etc")
            .clicked()
        {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
            ui.close_menu();
        }
    });
}
