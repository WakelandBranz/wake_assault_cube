use nvidia_overlay::core::{Overlay, OverlayError};
use serde::{Deserialize, Serialize};

use crate::cheat::{
    sdk::{
        GameState,
        player::Player,
    },
    features::{
        Feature,
        visuals::{
            get_color_rgba,
            render_context::RenderContext,
        },
    },
};

// BoxEsp feature with UI integration
#[derive(Serialize, Deserialize, Clone)]
pub struct NameEsp {
    // I'd like to remove enabled in the future so that when all features are being iterated through
    // and rendered or updated I can save on an if statement, but that'll be for another time.
    // Just gotta get stuff working for now.
    pub enabled: bool,
    pub color: [f32; 4],
    pub y_offset: f32, // Adjust text position above head
}

impl Default for NameEsp {
    fn default() -> Self {
        Self {
            enabled: false,
            color: [255.0, 255.0, 255.0, 255.0],  // White RGBA
            y_offset: 5.0,  // Offset above head
        }
    }
}

impl Feature for NameEsp {

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Never used for NameEsp
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
        let text_width = overlay.get_text_width(player.name())?;
        overlay.draw_text(
            (
                render_ctx.head_screen_pos.x - (text_width / 2) as f32, // Adjust for text centering
                render_ctx.head_screen_pos.y + self.y_offset
            ),
            player.name(),
            get_color_rgba(self.color)
        )
    }
}
