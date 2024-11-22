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
        Self {
        }
    }
}

impl TabVisuals {
    pub fn render(&mut self, ui: &mut egui::Ui, config: Arc<RwLock<Config>>) {
        let mut config = config.write().unwrap();

        let box_esp = &mut config.features.visuals.box_esp;

        // Checkbox for enabling/disabling the feature
        ui.checkbox(&mut box_esp.enabled, "Box ESP").clicked();

        // Collapsing header for settings (only shown when enabled)
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

        let name_esp = &mut config.features.visuals.name_esp;

        // Checkbox for enabling/disabling the feature
        ui.checkbox(&mut name_esp.enabled, "Name ESP").clicked();

        // Collapsing header for settings (only shown when enabled)
        if name_esp.is_enabled() {
            egui::CollapsingHeader::new("Name ESP Settings")
                .open(Some(name_esp.enabled))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        ui.color_edit_button_rgba_unmultiplied(&mut name_esp.color);
                    });

                    // Option 2: Using Slider
                    ui.add(egui::Slider::new(&mut name_esp.y_offset, -50.0..=50.0)
                        .text("Y Offset")
                        .suffix("px"));
                });
        }
    }
}