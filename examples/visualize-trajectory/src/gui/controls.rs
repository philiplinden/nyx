use std::fmt;

use crate::gui::View;
use crate::scenarios::Scenario;

pub struct ScenarioPicker {
    scenario: Scenario,
}

impl Default for ScenarioPicker {
    fn default() -> Self {
        Self {
            scenario: Scenario::LunarTransfer,
        }
    }
}

/// Dropdown list to select reference frame
impl View for ScenarioPicker {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // change the scenario enum from a dropdown
            egui::ComboBox::from_label("Scenario")
                .selected_text(self.scenario.to_string())
                .show_ui(ui, |ui| {
                    for scen in &[
                        Scenario::LunarTransfer,
                        Scenario::OrbitDesign,
                        // Scenario::IntlSpaceStation,
                    ] {
                        ui.selectable_value(&mut self.scenario, *scen, scen.to_string());
                    }
                });
            // run the scenario when button is clicked
            if ui.button("run").clicked() {
                self.scenario.run()
            }
        });
    }
}

impl fmt::Display for ScenarioPicker {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.scenario.fmt(f)
    }
}
