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
use crate::cheat::features::visuals::Position;

// BoxEsp feature with UI integration
#[derive(Serialize, Deserialize, Clone)]
pub struct HealthbarESP {
    // I'd like to remove enabled in the future so that when all features are being iterated through
    // and rendered or updated I can save on an if statement, but that'll be for another time.
    // Just gotta get stuff working for now.
    pub enabled: bool,
    pub thickness: f32,
    pub top_down: bool,
    pub position: Position,
}

impl Default for HealthbarESP {
    fn default() -> Self {
        Self {
            enabled: false,
            top_down: false,
            thickness: 0.050,
            position: Position::Right
            //x_offset: 27.0,
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

        // Calculate dimensions and positions based on orientation
        let (x_pos, y_pos, bar_width, bar_height, health_width, health_height) = match self.position {
            Position::Top => {
                let bar_width = render_ctx.player_width;
                let bar_height = render_ctx.player_height * self.thickness;
                let health_width = bar_width * (player.health as f32 / 100.0);

                (
                    render_ctx.head_screen_pos.x - (bar_width / 2.0),
                    render_ctx.head_screen_pos.y - bar_height - 5.0,
                    bar_width,
                    bar_height,
                    health_width,
                    bar_height
                )
            },
            Position::Bottom => {
                let bar_width = render_ctx.player_width;
                let bar_height = render_ctx.player_height * self.thickness;
                let health_width = bar_width * (player.health as f32 / 100.0);

                (
                    render_ctx.head_screen_pos.x - (bar_width / 2.0),
                    render_ctx.feet_screen_pos.y + 5.0,
                    bar_width,
                    bar_height,
                    health_width,
                    bar_height
                )
            },
            Position::Left => {
                let bar_width = render_ctx.player_width * self.thickness;
                let bar_height = render_ctx.player_height;
                let health_height = bar_height * (player.health as f32 / 100.0);

                (
                    render_ctx.head_screen_pos.x - (render_ctx.player_width / 2.0) - bar_width - 2.5,
                    render_ctx.head_screen_pos.y,
                    bar_width,
                    bar_height,
                    bar_width,
                    health_height
                )
            },
            Position::Right => {
                let bar_width = render_ctx.player_width * self.thickness;
                let bar_height = render_ctx.player_height;
                let health_height = bar_height * (player.health as f32 / 100.0);

                (
                    render_ctx.head_screen_pos.x + (render_ctx.player_width / 2.0) + 2.5,
                    render_ctx.head_screen_pos.y,
                    bar_width,
                    bar_height,
                    bar_width,
                    health_height
                )
            },
            _ => unreachable!()
        };

        // Draw black background bar (always full size)
        overlay.draw_filled_rect(
            (x_pos, y_pos),
            (bar_width, bar_height),
            (0, 0, 0, 255)
        )?;

        // Draw health bar on top (sized according to health percentage)
        // Draw health bar
        let health_y_pos = if self.position == Position::Left && self.top_down || self.position == Position::Right && self.top_down {
            y_pos
        }
        else {
            y_pos + (bar_height - health_height)
        };

        overlay.draw_filled_rect(
            (x_pos, health_y_pos),
            (health_width, health_height),
            get_color_rgba(healthbar_color)
        )
    }
}
