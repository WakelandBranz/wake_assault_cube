use nvidia_amd_overlay::core::{Overlay, OverlayError};
use serde::{Deserialize, Serialize};

use crate::cheat::{
    sdk::{
        GameState,
        player::Player,
    },
    features::{
        Feature,
        get_color_rgba,
        visuals::{
            render_context::RenderContext,
        }
    },
};

// BoxEsp feature with UI integration
#[derive(Serialize, Deserialize, Clone)]
pub struct HeadEsp {
    // I'd like to remove enabled in the future so that when all features are being iterated through
    // and rendered or updated I can save on an if statement, but that'll be for another time.
    // Just gotta get stuff working for now.
    pub enabled: bool,
    pub color: [f32; 4],
    pub outline_color: [f32; 4],
    pub thickness: f32,
    pub is_filled: bool,
    pub fill_color1: [f32; 4],
    pub fill_color2: [f32; 4],
    pub is_radial: bool,
}

impl Default for HeadEsp {
    fn default() -> Self {
        Self {
            enabled: false,
            color: [255.0, 255.0, 255.0, 255.0],
            outline_color: [0.0, 0.0, 0.0, 255.0],
            thickness: 1.0,
            is_filled: false,
            fill_color1: [255.0, 255.0, 255.0, 255.0],
            fill_color2: [255.0, 255.0, 255.0, 255.0],
            is_radial: false,
        }
    }
}

impl Feature for HeadEsp {

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Never used for render features
    fn update(&mut self, _game_ctx: &GameState) -> bool {
        false
    }

    fn update_settings(&mut self) -> bool {
        todo!()
    }

    fn render(
        &self,
        _player: &Player,
        render_ctx: &RenderContext,
        overlay: &mut Overlay
    ) -> Result<(), OverlayError> {
        if !self.is_enabled() {
            return Ok(());
        }

        // Calculate distance-based scaling
        let distance = render_ctx.distance;
        let scaled_radius = (25.0 / (distance * 0.1)).min(75.0); // Prevents too large scaling

        // Draw gradient fill first if enabled
        if self.is_filled {
            overlay.draw_gradient_circle(
                (render_ctx.head_screen_pos.x, render_ctx.head_screen_pos.y),
                scaled_radius - self.thickness, // Reduce radius to prevent overflow
                get_color_rgba(self.fill_color1),
                get_color_rgba(self.fill_color2),
                self.is_radial,
            )?;
        }

        // Draw outline using a single thicker circle
        overlay.draw_circle(
            (render_ctx.head_screen_pos.x, render_ctx.head_screen_pos.y),
            scaled_radius + self.thickness, // Outer circle (outline)
            self.thickness * 2.0,
            get_color_rgba(self.outline_color)
        )?;

        // Draw main circle
        overlay.draw_circle(
            (render_ctx.head_screen_pos.x, render_ctx.head_screen_pos.y),
            scaled_radius,
            self.thickness,
            get_color_rgba(self.color)
        )?;

        Ok(())
    }
}
