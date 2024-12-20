use glam::Vec2;

// Common data needed for rendering
#[derive(Debug)]
pub struct RenderContext {
    pub head_screen_pos: Vec2,
    pub feet_screen_pos: Vec2,
    pub player_height: f32,
    pub player_width: f32,
    pub screen_bounds: Vec2,
    pub distance: f32,  // Distance to player, useful for scaling
}