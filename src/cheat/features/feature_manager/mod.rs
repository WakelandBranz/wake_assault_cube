use std::sync::{Arc, RwLock};
use eframe::egui::Vec2;
use nvidia_overlay::core::{Overlay, OverlayError};
use crate::cheat::{
    features::{
        Feature,
        visuals::render_context::RenderContext,
    },
    sdk::GameState,
};
use crate::config::Config;

extern crate fps_counter;
use fps_counter::*;

pub struct FeatureManager {
    overlay: Overlay,
    game_ctx: Arc<RwLock<GameState>>,
    config: Arc<RwLock<Config>>,
    fps_counter: FPSCounter,
}

impl FeatureManager {
    /// OVERLAY MUST BE INITIALIZED PRIOR TO PASSING INTO THIS!
    pub fn new(overlay: Overlay, game_ctx: Arc<RwLock<GameState>>, config: Arc<RwLock<Config>>) -> Self {
        Self {
            overlay,
            game_ctx,
            config,
            fps_counter: FPSCounter::default(),
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

        // Unwrap main structs
        let game_ctx = self.game_ctx.read().unwrap();
        let config = self.config.read().unwrap();

        // Unwrap feature structs
        let visuals = &config.features.visuals;
        let misc = &config.features.misc;

        for player in &game_ctx.entity_list.entities {
            // Skip if entity is not valid
            if !player.is_alive() {
                continue;
            }

            // Get head position first
            let Some(head_screen_pos) = game_ctx.world_to_screen(
                player.pos_head.into(),
                false
            )
            else {
                continue
            };

            // Get feet position if needed
            let Some(feet_screen_pos) = game_ctx.world_to_screen(
                player.pos.into(),
                false
            )
            else {
                continue
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

            // Render features (features check themselves if they are enabled)
            visuals.box_esp.render(&player, &render_ctx, &mut self.overlay)?;
            visuals.name_esp.render(&player, &render_ctx, &mut self.overlay)?;
            visuals.healthbar_esp.render(&player, &render_ctx, &mut self.overlay)?;
        }
        // Get framerate by counting once each loop.
        misc.fps.display(self.fps_counter.tick(), &mut self.overlay)?;

        self.overlay.end_scene();
        Ok(())
    }

    // Verifies all necessary checks to see if the cheat should run a tick
    pub fn should_tick(&mut self) -> bool {
        // No need to run a tick if the game isn't maximized
        if !self.game_ctx.read().unwrap().process.is_focused() {
            self.overlay.force_clear_scene();
            return false;
        };

        let visuals = &self.config.read().unwrap().features.visuals;
        let misc = &self.config.read().unwrap().features.misc;

        // Check if any features are enabled before doing transforms
        if !visuals.box_esp.is_enabled()
            && !visuals.name_esp.is_enabled()
            && !visuals.healthbar_esp.is_enabled()
            && !misc.fps.is_enabled()
        {
            return false;
        }

        true
        // Add checks as necessary
    }
}