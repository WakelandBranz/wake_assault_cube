// Not sure if visuals is the best name for this file. Egui already has some implementations
// named visuals. For my purposes though, I do like the name visuals for an ESP tab.

use std::sync::{Arc, RwLock};
use eframe::egui;
use serde::{Deserialize, Serialize};
use crate::cheat::features::Feature;
use crate::cheat::features::visuals::Position;
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

                        ui.horizontal(|ui| {
                            ui.label("Outline Color:");
                            ui.color_edit_button_rgba_unmultiplied(&mut box_esp.outline_color);
                        });

                        ui.add(egui::Slider::new(&mut box_esp.thickness, 1.0..=5.0)
                            .text("Thickness"));

                        ui.checkbox(&mut box_esp.is_filled, "Filled").clicked();

                        if box_esp.is_filled {
                            ui.horizontal(|ui| {
                                ui.label("Fill color 1:");
                                ui.color_edit_button_rgba_unmultiplied(&mut box_esp.fill_color1);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Fill color 2:");
                                ui.color_edit_button_rgba_unmultiplied(&mut box_esp.fill_color2);
                            });
                            ui.checkbox(&mut box_esp.is_vertical, "Vertical Fill").clicked();
                        }
                    });
            }
        }

        // Head ESP
        {
            let head_esp = &mut visuals.head_esp;
            ui.checkbox(&mut head_esp.enabled, "Head ESP").clicked();

            if head_esp.is_enabled() {
                egui::CollapsingHeader::new("Head ESP Settings")
                    .open(Some(head_esp.enabled))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Color:");
                            ui.color_edit_button_rgba_unmultiplied(&mut head_esp.color);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Outline color:");
                            ui.color_edit_button_rgba_unmultiplied(&mut head_esp.outline_color);
                        });

                        ui.add(egui::Slider::new(&mut head_esp.thickness, 1.0..=5.0)
                            .text("Thickness"));

                        ui.checkbox(&mut head_esp.is_filled, "Filled").clicked();

                        if head_esp.is_filled {
                            ui.horizontal(|ui| {
                                ui.label("Fill color 1:");
                                ui.color_edit_button_rgba_unmultiplied(&mut head_esp.fill_color1);
                            });
                            ui.horizontal(|ui| {
                                ui.label("Fill color 2:");
                                ui.color_edit_button_rgba_unmultiplied(&mut head_esp.fill_color2);
                            });
                            ui.checkbox(&mut head_esp.is_radial, "Radial").clicked();
                        }
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

                        egui::ComboBox::from_label("Text Position")
                            .selected_text(match name_esp.position {
                                Position::Top => "Top",
                                Position::Bottom => "Bottom",
                                _ => unreachable!()
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut name_esp.position, Position::Top, "Top");
                                ui.selectable_value(&mut name_esp.position, Position::Bottom, "Bottom");
                            });
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
                        ui.add(egui::Slider::new(&mut healthbar_esp.thickness, 0.01..=0.05)
                            .text("Thickness"));

                        egui::ComboBox::from_label("Healthbar Position")
                            .selected_text(match healthbar_esp.position {
                                Position::Top => "Top",
                                Position::Bottom => "Bottom",
                                Position::Left => "Left",
                                Position::Right => "Right",
                                _ => unreachable!()
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut healthbar_esp.position, Position::Top, "Top");
                                ui.selectable_value(&mut healthbar_esp.position, Position::Bottom, "Bottom");
                                ui.selectable_value(&mut healthbar_esp.position, Position::Left, "Left");
                                ui.selectable_value(&mut healthbar_esp.position, Position::Right, "Right");
                            });

                        // Only show top_down checkbox for Left or Right positions
                        if matches!(healthbar_esp.position, Position::Left | Position::Right) {
                            ui.checkbox(&mut healthbar_esp.top_down, "Top Down").clicked();
                        }
                    });
            }
        }
    }
}