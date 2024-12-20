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
pub struct HealthbarESP {
    // I'd like to remove enabled in the future so that when all features are being iterated through
    // and rendered or updated I can save on an if statement, but that'll be for another time.
    // Just gotta get stuff working for now.
    pub enabled: bool,
    pub thickness: f32,
    pub width_scale: f32,
    pub x_offset: f32,
}

impl Default for HealthbarESP {
    fn default() -> Self {
        Self {
            enabled: false,
            thickness: 0.32,
            width_scale: 0.02,
            x_offset: 27.0,
        }
    }
}

impl Feature for HealthbarESP {

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

    // Thank you https://www.unknowncheats.me/forum/direct3d/59782-healthbar-esp-source.html
    fn render(
        &self,
        player: &Player,
        render_ctx: &RenderContext,
        overlay: &mut Overlay
    ) -> Result<(), OverlayError> {
        // Don't run if not enabled
        if !self.is_enabled() {
            return Ok(())
        }

        // Calculate health-based color (fixed scaling)
        let green = (player.health as f32 * 2.55) / 255.0;  // normalize to 0.0-1.0
        let red = 1.0 - green;  // Inverse in 0.0-1.0 range
        let healthbar_color: [f32; 4] = [red, green, 0.0, 1.0];

        let height = render_ctx.feet_screen_pos.y - render_ctx.head_screen_pos.y;
        let width = height * self.width_scale;

        let scaled_x_offset = self.x_offset * (height / 100.0);

        // Position calculation for both rectangles
        let x_pos = render_ctx.head_screen_pos.x + scaled_x_offset;
        let y_pos = render_ctx.head_screen_pos.y;

        // Outline rectangle
        overlay.draw_filled_rect(
            (x_pos, y_pos),
            (width, height),
            (0, 0, 0, 255)
        )?;

        // Calculate health-scaled height
        let health_height = height * (player.health as f32 / 100.0);

        // Overlay.draw will return a Result<(), OverlayError>, so we can just use that to propagate
        // Primary rectangle
        overlay.draw_filled_rect(
            (x_pos, y_pos),
            (width, health_height),
            get_color_rgba(healthbar_color)
        )
    }
}
