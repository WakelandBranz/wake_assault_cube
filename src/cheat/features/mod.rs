// Heavily derived from https://github.com/Valthrun/Valthrun/blob/master/controller/src/enhancements/mod.rs
// Thank you Valthrun, huge help for publishing your project.

use serde::{Deserialize, Serialize};

use nvidia_overlay::core::{Overlay, OverlayError};
use crate::cheat::{
    features::visuals::{
        Visuals,
        render_context::RenderContext,
    },
    sdk::{
        GameState,
        player::Player
    }
};


pub mod visuals;
pub mod view;
pub mod feature_manager;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Features {
    pub(crate) visuals: Visuals,
}

// Core feature trait - just the basics
/// Implement for each feature!
pub trait Feature {
    // Simple check to see if the feature is enabled
    fn is_enabled(&self) -> bool;

    // Used for features like aimbot that DON'T need to be rendered.
    fn update(&mut self, game_ctx: &GameState) -> bool;

    // Updates the struct fields
    fn update_settings(&mut self) -> bool {
        todo!()
    }

    // Used for features like ESP that DO need to be rendered.
    fn render(&self, entity: &Player, render_ctx: &RenderContext, overlay: &mut Overlay) -> Result<(), OverlayError>;

    // Can be helpful for quick debug stuff
    fn render_debug(&mut self, _game_ctx: &GameState) -> Result<(), OverlayError> {
        Ok(())
    }
}




