mod visuals;
mod misc;
mod tab_selector;
mod menu_config;
mod style;

use std::{
    thread::Thread,
    sync::{
        Arc,
        RwLock,
        atomic::AtomicBool,
    },
};
use std::sync::atomic::Ordering;
use eframe::{
    egui,
    egui::Context,
    Frame,
    Storage,
};
use nvidia_overlay::core::Overlay;
use serde::{Deserialize, Serialize};
use crate::config::*;
use crate::menu::gui::{
    tab_selector::*,
    visuals::TabVisuals,
    misc::TabMisc,
    menu_config::TabMenuConfig
};

#[derive(Serialize, Deserialize)]
pub struct Menu {
    config: Arc<RwLock<Config>>,
    tab_selector: TabSelector,
    tab_visuals: TabVisuals,
    tab_misc: TabMisc,
    tab_menu_config: TabMenuConfig,
    #[serde(skip)]
    process_running: Arc<AtomicBool>
}

impl Menu {
    /// Called once before the first frame.
    pub fn new(config: Arc<RwLock<Config>>, process_running: Arc<AtomicBool>, cc: &eframe::CreationContext) -> Self {

        match style::set_custom_visuals(cc) {
            Ok(()) => log::debug!("Successfully set menu visual settings."),
            Err(error) => log::debug!("Unsuccessfully set menu visual settings: {:?}", error)
        }

        // Try to load the previous tab from storage
        let tab_selector = cc.storage
            .and_then(|storage| eframe::get_value(storage, "tab_selector"))
            .unwrap_or(TabSelector::default());

        // Try to load all previous tab data
        let tab_visuals = cc.storage
            .and_then(|storage| eframe::get_value(storage, "tab_visuals"))
            .unwrap_or_default();

        let tab_misc = cc.storage
            .and_then(|storage| eframe::get_value(storage, "tab_misc"))
            .unwrap_or(TabMisc::default());

        let tab_menu_config = cc.storage
            .and_then(|storage| eframe::get_value(storage, "tab_menu_config"))
            .unwrap_or(TabMenuConfig::default());

        // Otherwise create new state
        Self {
            config: config.clone(),
            tab_selector,
            tab_visuals,
            tab_misc,
            tab_menu_config,
            process_running,
        }
    }

    fn render_content(&mut self, ctx: &egui::Context) {
        // Render the tab selector
        self.tab_selector.show(ctx);

        // Render the content based on selected tab
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab_selector.selected_tab() {
                Tab::Visuals => self.tab_visuals.render(ui, self.config.clone()),
                Tab::Misc => self.tab_misc.render(ui, self.config.clone()),
                Tab::MenuConfig => self.render_menu_config(ui),
            }
        });
    }

    fn render_misc(&mut self, ui: &mut egui::Ui) {
        ui.heading("Misc");
        // Add settings content
    }

    fn render_menu_config(&mut self, ui: &mut egui::Ui) {
        ui.heading("Menu/Config");
        // Add profile content
    }
}

// Main rendering loop
impl eframe::App for Menu {

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Ensure that config is parsed before running first render loop!

        // Main render loop
        self.render_content(ctx);

    }

    fn save(&mut self, storage: &mut dyn Storage) {
        // Save the config
        self.config.write().unwrap().save().expect("Failed to save config on exit!");

        // Save UI state
        eframe::set_value(storage, "tab_selector", &self.tab_selector);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        log::info!("Application exiting...");

        // Signal threads to stop
        self.process_running.store(false, Ordering::Relaxed);

        // Note: eframe calls save on exit
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(120)
    }
}