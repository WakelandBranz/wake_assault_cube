use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Tab {
    Visuals,
    Misc,
    MenuConfig,
    // Add more tabs as needed
}

impl Tab {
    // Helper function to get all tabs
    pub fn all() -> Vec<Tab> {
        vec![Tab::Visuals, Tab::Misc, Tab::MenuConfig]
    }

    // Convert tab to display string
    fn to_string(&self) -> &'static str {
        match self {
            Tab::Visuals => "Visuals",
            Tab::Misc => "Misc",
            Tab::MenuConfig => "Menu/Config",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TabSelector {
    selected_tab: Tab,
    tab_height: f32,
    show_separator: bool,
}

impl Default for TabSelector {
    fn default() -> Self {
        Self {
            selected_tab: Tab::MenuConfig,
            tab_height: 32.0,
            show_separator: true,
        }
    }
}

impl TabSelector {
    pub fn new() -> Self {
        Self::default()
    }

    // Builder pattern methods for customization
    pub fn with_initial_tab(mut self, tab: Tab) -> Self {
        self.selected_tab = tab;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.tab_height = height;
        self
    }

    pub fn with_separator(mut self, show: bool) -> Self {
        self.show_separator = show;
        self
    }

    // Getter for currently selected tab
    pub fn selected_tab(&self) -> Tab {
        self.selected_tab
    }

    // Main rendering function
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tab_selector")
            .exact_height(self.tab_height)
            .show(ctx, |ui| {
                ui.add_space(2.0);
                self.render_tabs(ui);
                ui.add_space(2.0);
            });
    }

    fn render_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            let style = ui.style_mut();
            style.spacing.item_spacing.x = 15.0;

            let available_width = ui.available_width();
            let tab_count = Tab::all().len() as f32;
            let tab_width = available_width / tab_count;

            // Add all tabs
            for tab in Tab::all() {
                let selected = self.selected_tab == tab;
                let button = egui::SelectableLabel::new(
                    selected,
                    tab.to_string(),
                );

                let response = ui
                    .allocate_ui(egui::vec2(tab_width, ui.available_height()), |ui| {
                        // Center the text within each tab
                        ui.centered_and_justified(|ui| {
                            ui.add(button)
                        }).inner
                    })
                    .inner;

                if response.clicked() {
                    self.selected_tab = tab;
                }

                // Custom styling for the selected tab
                if selected {
                    let rect = response.rect;
                    let stroke = ui.style().visuals.widgets.active.bg_stroke;

                    if self.show_separator {
                        // Draw bottom border for selected tab
                        let bottom_line = egui::Shape::line(
                            vec![
                                rect.left_bottom(),
                                rect.right_bottom(),
                            ],
                            stroke,
                        );
                        ui.painter().add(bottom_line);
                    }
                }
            }
        });
    }
}