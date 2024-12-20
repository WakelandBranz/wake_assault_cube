use std::sync::{Arc, RwLock};
use eframe::egui;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use crate::cheat::features::Feature;
use crate::config::Config;

#[derive(Serialize, Deserialize)]
pub struct TabMisc {}

impl Default for TabMisc {
    fn default() -> Self {
        Self {}
    }
}

impl TabMisc {
    pub fn render(&mut self, ui: &mut egui::Ui, config: Arc<RwLock<Config>>) {
        let mut config = config.write().unwrap();

        let mut fps = &mut config.features.misc.fps;

        // Checkbox for enabling/disabling the feature
        ui.checkbox(&mut fps.enabled, "FPS Counter").clicked();

        // Collapsing header for settings (only shown when enabled)
        if fps.is_enabled() {
            egui::CollapsingHeader::new("FPS Settings")
                .open(Some(fps.enabled))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        ui.color_edit_button_rgba_unmultiplied(&mut fps.color);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Position:");
                        ui.add(egui::Slider::new(&mut fps.pos[0], 0.0..=1920.0)
                            .text("X"));
                        ui.add(egui::Slider::new(&mut fps.pos[1], 0.0..=1080.0)
                            .text("Y"));
                    });

                    // Add a reset button
                    if ui.button("Reset Position").clicked() {
                        fps.pos = [100.0, 100.0]; // Default position
                    }
                });
        }
    }
}