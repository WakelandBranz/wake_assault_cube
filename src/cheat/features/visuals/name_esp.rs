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
        },
    },
};
use crate::cheat::features::visuals::Position;

// BoxEsp feature with UI integration
#[derive(Serialize, Deserialize, Clone)]
pub struct NameEsp {
    // I'd like to remove enabled in the future so that when all features are being iterated through
    // and rendered or updated I can save on an if statement, but that'll be for another time.
    // Just gotta get stuff working for now.
    pub enabled: bool,
    pub color: [f32; 4],
    pub position: Position,
}

impl Default for NameEsp {
    fn default() -> Self {
        Self {
            enabled: false,
            color: [255.0, 255.0, 255.0, 255.0],  // White RGBA
            position: Position::Top
        }
    }
}

impl Feature for NameEsp {

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
        player: &Player,
        render_ctx: &RenderContext,
        overlay: &mut Overlay
    ) -> Result<(), OverlayError> {
        if !self.is_enabled() {
            return Ok(())
        }

        let text_width = overlay.get_text_width(player.name())? as f32;

        let (x_pos, y_pos) = match self.position {
            Position::Top => (
                render_ctx.head_screen_pos.x - (text_width / 2.0),
                 render_ctx.head_screen_pos.y - 15.0
            )
            ,
            Position::Bottom => (
                render_ctx.head_screen_pos.x - (text_width / 2.0),
                render_ctx.feet_screen_pos.y
            )
            ,
            _ => unreachable!()
        };


        overlay.draw_outlined_text(
            (
                x_pos, // Simple centering
                y_pos
            ),
            player.name().as_str(),
            get_color_rgba(self.color)
        )
    }
}
