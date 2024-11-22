use std::sync::{Arc, RwLock};
use nvidia_overlay::core::{Overlay, OverlayError};
use crate::cheat::{
    features::{
        Feature,
        visuals::render_context::RenderContext,
    },
    sdk::GameState,
};
use crate::config::Config;

pub struct FeatureManager {
    overlay: Overlay,
    game_ctx: Arc<RwLock<GameState>>,
    config: Arc<RwLock<Config>>
}

impl FeatureManager {
    /// OVERLAY MUST BE INITIALIZED PRIOR TO PASSING INTO THIS!
    pub fn new(overlay: Overlay, game_ctx: Arc<RwLock<GameState>>, config: Arc<RwLock<Config>>) -> Self {
        Self {
            overlay,
            game_ctx,
            config
        }
    }

    pub fn tick(&mut self) -> Result<(), OverlayError> {
        // Ensure that all conditions are met to actually run features
        if !self.should_tick() {
            return Ok(());
        }

        // Prepare screen for next render
        self.overlay.begin_scene();
        self.overlay.clear_scene();

        let game_ctx = self.game_ctx.read().unwrap();
        let config = self.config.read().unwrap();

        let visuals = config.clone().features.visuals;
        for player in game_ctx.entity_list.entities.clone() {
            // Skip if entity is not valid
            if !player.is_alive() {
                continue;
            }

            // Check if any features are enabled before doing transforms
            if !visuals.box_esp.is_enabled() && !visuals.name_esp.is_enabled() {
                continue;
            }

            // Get head position first
            let head_screen_pos = match game_ctx.world_to_screen(
                player.pos_head.into(),
                false
            )
            {
                Some(pos) => pos,
                None => continue, // Skip if not visible
            };

            // Get feet position if needed
            let feet_screen_pos = match game_ctx.world_to_screen(
                player.pos.into(),
                false
            )
            {
                Some(pos) => pos,
                None => continue,
            };

            // Calculate distance (useful for scaling)
            let distance = (game_ctx.local_player.position() - player.pos).length();

            // Create render context with shared data
            let render_ctx = RenderContext {
                head_screen_pos,
                feet_screen_pos,
                screen_bounds: game_ctx.screen_bounds,
                distance,
            };

            // Render enabled features
            if visuals.box_esp.is_enabled() {
                visuals.box_esp.render(&player, &render_ctx, &mut self.overlay)?;
            }

            if visuals.name_esp.is_enabled() {
                visuals.name_esp.render(&player, &render_ctx, &mut self.overlay)?;
            }
        }
        self.overlay.end_scene();
        Ok(())
    }

    // Verifies all necessary checks to see if the cheat should run a tick
    pub fn should_tick(&self) -> bool {
        // No need to run a tick if the game isn't maximized
        if !self.game_ctx.read().unwrap().process.is_focused() {
            return false;
        };

        true
        // Add checks as necessary
    }

    pub fn cleanup(&mut self) {
        // This might cause a cleanup twice...
        self.overlay.cleanup();
    }
}