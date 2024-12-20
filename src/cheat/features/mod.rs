// Heavily derived from https://github.com/Valthrun/Valthrun/blob/master/controller/src/enhancements/mod.rs
// Thank you Valthrun, huge help for publishing your project.

use serde::{Deserialize, Serialize};

use nvidia_amd_overlay::core::{Overlay, OverlayError};
use crate::cheat::{
    features::{
        visuals::{
            Visuals,
            render_context::RenderContext,
        },
        misc::{
            Misc,
        }
    },
    sdk::{
        GameState,
        player::Player
    }
};

pub mod visuals;
pub mod view;
pub mod feature_manager;
mod misc;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Features {
    pub(crate) visuals: Visuals,
    pub(crate) misc: Misc,
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

fn get_color_rgba(color: [f32; 4]) -> (u8, u8, u8, u8) {
    (
        (color[0] * 255.0) as u8,
        (color[1] * 255.0) as u8,
        (color[2] * 255.0) as u8,
        (color[3] * 255.0) as u8,
    )
}

