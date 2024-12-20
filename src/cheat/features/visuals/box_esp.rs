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
pub struct BoxEsp {
    // I'd like to remove enabled in the future so that when all features are being iterated through
    // and rendered or updated I can save on an if statement, but that'll be for another time.
    // Just gotta get stuff working for now.
    pub enabled: bool,
    pub color: [f32; 4],
    pub thickness: f32,
    pub width_scale: f32,
}

impl Default for BoxEsp {
    fn default() -> Self {
        Self {
            enabled: false,
            color: [255.0, 0.0, 0.0, 255.0],  // Red RGBA
            thickness: 1.0,
            width_scale: 0.5,
        }
    }
}

impl Feature for BoxEsp {

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
        _player: &Player, // Not needed for box rendering
        render_ctx: &RenderContext,
        overlay: &mut Overlay
    ) -> Result<(), OverlayError> {
        // Don't run if not enabled
        if !self.is_enabled() {
            return Ok(())
        }

        let height = render_ctx.feet_screen_pos.y - render_ctx.head_screen_pos.y;
        let width = height * self.width_scale;

        // Outline rectangle
        overlay.draw_rect(
            (render_ctx.head_screen_pos.x - width / 2.0, render_ctx.head_screen_pos.y),
            (width, height),
            self.thickness * 2.5, // Extra thickness borders both sides of primary rectangle
            (0, 0, 0, 255)
        )?;

        // Overlay.draw will return a Result<(), OverlayError>, so we can just use that to propagate
        // Primary rectangle
        overlay.draw_rect(
            (render_ctx.head_screen_pos.x - width / 2.0, render_ctx.head_screen_pos.y),
            (width, height),
            self.thickness,
            get_color_rgba(self.color)
        )
    }
}
