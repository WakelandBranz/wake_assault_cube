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

// BoxEsp feature with UI integration
#[derive(Serialize, Deserialize, Clone)]
pub struct FPS {
    // I'd like to remove enabled in the future so that when all features are being iterated through
    // and rendered or updated I can save on an if statement, but that'll be for another time.
    // Just gotta get stuff working for now.
    pub enabled: bool,
    pub color: [f32; 4],
    pub pos: [f32; 2],
}

impl Default for FPS {
    fn default() -> Self {
        Self {
            enabled: false,
            color: [0.0, 255.0, 0.0, 255.0],  // Green RGBA
            pos: [1755.0, 280.0],
        }
    }
}

impl Feature for FPS {

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

    /// This function should never be used for FPS even though it is a Feature. FPS requires
    /// unique parameters
    fn render(
        &self,
        _player: &Player,
        _render_ctx: &RenderContext,
        _overlay: &mut Overlay
    ) -> Result<(), OverlayError> {
        Ok(())
    }
}

impl FPS {
    pub(crate) fn display(
        &self,
        fps: usize, // FPSCounter.tick() returns a usize
        overlay: &mut Overlay
    ) -> Result<(), OverlayError> {
        if !self.is_enabled() {
            return Ok(());
        }

        overlay.draw_outlined_text(
            (self.pos[0], self.pos[1]),
            format!("FPS: {}", fps).as_str(),
            get_color_rgba(self.color))?;
        Ok(())
    }
}