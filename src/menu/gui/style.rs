use eframe::egui;

// This is temporary, not sure what to do with it yet.
#[derive(Debug)]
pub enum CustomVisualsError {
    FailedToSetCustomVisuals,
}

// Here is where all visuals should be customized for the menu.
pub fn set_custom_visuals(cc: &eframe::CreationContext) -> Result<(), CustomVisualsError> {
    // Set custom visuals
    let mut visuals = egui::Visuals::default();

    // Customize visuals
    visuals.window_rounding = 4.0.into();
    visuals.window_shadow.spread = 8.0;
    visuals.popup_shadow.spread = 4.0;
    visuals.menu_rounding = 4.0.into();
    visuals.button_frame = true;
    visuals.window_fill = egui::Color32::from_rgb(32, 32, 32);

    cc.egui_ctx.set_visuals(visuals);

    Ok(())
}