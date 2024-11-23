// Not sure if visuals is the best name for this file. Egui already has some implementations
// named visuals. For my purposes though, I do like the name visuals for an ESP tab.

use std::sync::{Arc, RwLock};
use eframe::egui;
use serde::{Deserialize, Serialize};
use crate::cheat::features::Feature;
use crate::config::Config;


// This struct is the main TabVisuals that combines settings and runtime config
#[derive(Serialize, Deserialize)]
pub struct TabVisuals {}

impl Default for TabVisuals {
    fn default() -> Self {
        Self {}
    }
}

impl TabVisuals {
    pub fn render(&mut self, ui: &mut egui::Ui, config: Arc<RwLock<Config>>) {
        let mut config = config.write().unwrap();
        let visuals = &mut config.features.visuals;

        // Box ESP
        {
            let box_esp = &mut visuals.box_esp;
            ui.checkbox(&mut box_esp.enabled, "Box ESP").clicked();

            if box_esp.is_enabled() {
                egui::CollapsingHeader::new("Box ESP Settings")
                    .open(Some(box_esp.enabled))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Color:");
                            ui.color_edit_button_rgba_unmultiplied(&mut box_esp.color);
                        });

                        ui.add(egui::Slider::new(&mut box_esp.thickness, 1.0..=5.0)
                            .text("Thickness"));

                        ui.add(egui::Slider::new(&mut box_esp.width_scale, 0.0..=5.0)
                            .text("Width Scale"));
                    });
            }
        }

        // Name ESP
        {
            let name_esp = &mut visuals.name_esp;
            ui.checkbox(&mut name_esp.enabled, "Name ESP").clicked();

            if name_esp.is_enabled() {
                egui::CollapsingHeader::new("Name ESP Settings")
                    .open(Some(name_esp.enabled))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Color:");
                            ui.color_edit_button_rgba_unmultiplied(&mut name_esp.color);
                        });

                        ui.add(egui::Slider::new(&mut name_esp.y_offset, -50.0..=50.0)
                            .text("Y Offset")
                            .suffix("px"));
                    });
            }
        }

        // Healthbar ESP
        {
            let healthbar_esp = &mut visuals.healthbar_esp;
            ui.checkbox(&mut healthbar_esp.enabled, "Healthbar ESP").clicked();

            if healthbar_esp.is_enabled() {
                egui::CollapsingHeader::new("Healthbar ESP Settings")
                    .open(Some(healthbar_esp.enabled))
                    .show(ui, |ui| {
                        ui.add(egui::Slider::new(&mut healthbar_esp.thickness, 0.01..=1.0)
                            .text("Thickness"));

                        ui.add(egui::Slider::new(&mut healthbar_esp.width_scale, 0.01..=1.0)
                            .text("Width Scale"));

                        ui.add(egui::Slider::new(&mut healthbar_esp.x_offset, -200.0..=200.0)
                            .text("X Offset")
                            .suffix("px"));
                    });
            }
        }
    }
}