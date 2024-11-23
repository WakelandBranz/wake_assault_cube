use glam::Vec2;
use nvidia_overlay::core::{Overlay, OverlayError};
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
    pub pos: [u32; 2],
}

impl Default for FPS {
    fn default() -> Self {
        Self {
            enabled: false,
            color: [0.0, 255.0, 0.0, 255.0],  // Green RGBA
            pos: [1762, 280],
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
        pos: Vec2, // TODO! Replace this with window_context later!
        overlay: &mut Overlay
    ) -> Result<(), OverlayError> {
        if !self.is_enabled() {
            return Ok(());
        }

        overlay.draw_text(
            (pos.x, pos.y),
            format!("FPS: {}", fps),
            get_color_rgba(self.color))?;
        Ok(())
    }

    // Useful for rendering
    pub(crate) fn pos_to_vec2(&self, pos: [u32; 2]) -> Vec2 {
        Vec2::new(pos[0] as f32, pos[1] as f32)
    }
}