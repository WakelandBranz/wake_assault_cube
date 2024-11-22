use glam::{Vec2, Vec3, Vec4, Mat4, Vec4Swizzles};
use crate::cheat::{
    process::Process,
    sdk::{
        entity_list::EntityList,
        player::PlayerManager
    },
    features::visuals::render_context::RenderContext,
};


pub mod player;
pub mod weapon;
pub(crate) mod offsets;
pub mod entity_list;

/// Shared data structure that features might need
#[derive(Debug)]
pub struct GameState {
    pub process: Process,
    pub local_player: PlayerManager,
    pub entity_list: EntityList,
    pub screen_bounds: glam::Vec2,
    pub render_context: RenderContext,
}

impl GameState {
    pub fn new(process: Process, local_player: PlayerManager, entity_list: EntityList) -> Self {
        Self {
            process: process.clone(),
            local_player,
            entity_list,
            screen_bounds: process.get_screen_bounds().unwrap(),
            render_context: RenderContext {
                head_screen_pos: Default::default(),
                feet_screen_pos: Default::default(),
                screen_bounds: Default::default(),
                distance: 0.0,
            },
        }
    }

    pub fn update(&mut self) {
        self.local_player.update().expect("Failed to update local player!");
        self.entity_list.update().expect("Failed to update entity list!");
        self.screen_bounds = self.process.get_screen_bounds().expect("Failed to update screen bounds!");
    }

    // TODO! Benchmark this later to see which of the two world to screen functions are faster.
    pub fn _world_to_screen(&self, world_position: [f32; 3], should_clip: bool) -> Option<glam::Vec2> {
        // Get the current view matrix
        let view_matrix = self.local_player.view_matrix();

        // Convert world position to Vec4 for matrix multiplication (w = 1.0 for positions)
        let world_pos = glam::Vec4::new(world_position[0], world_position[1], world_position[2], 1.0);

        // Transform position by view matrix
        let clip_pos = view_matrix * world_pos;

        // Early exit if behind camera
        if clip_pos.w < 0.1 {
            return None;
        }

        // Perform perspective divide to get NDC coordinates
        let ndc = glam::Vec3::new(
            clip_pos.x / clip_pos.w,
            clip_pos.y / clip_pos.w,
            clip_pos.z / clip_pos.w
        );

        // Convert NDC to screen coordinates
        let screen_x = (self.screen_bounds.x * 0.5) * (1.0 + ndc.x);
        let screen_y = (self.screen_bounds.y * 0.5) * (1.0 - ndc.y);

        // Optional screen space clipping
        if should_clip {
            let margin = 0.0; // Increase this value to allow for off-screen positions
            if screen_x < -margin || screen_x > self.screen_bounds.x + margin ||
                screen_y < -margin || screen_y > self.screen_bounds.y + margin {
                return None;
            }
        }

        Some(glam::Vec2::new(screen_x, screen_y))
    }

    #[inline(always)]
    pub fn world_to_screen(&self, pos: [f32; 3], clip: bool) -> Option<Vec2> {
        // Skip Vec3 construction and directly make Vec4
        let clip_pos = self.local_player.view_matrix() * Vec4::new(pos[0], pos[1], pos[2], 1.0);

        // Early w check
        if clip_pos.w < 0.001 {
            return None;
        }

        // Calculate NDC x/y with single division
        let inv_w = 1.0 / clip_pos.w;
        let ndc_x = clip_pos.x * inv_w;
        let ndc_y = clip_pos.y * inv_w;

        // Early clip check before expensive screen space calculations
        if clip && (ndc_x < -1.0 || ndc_x > 1.0 || ndc_y < -1.0 || ndc_y > 1.0) {
            return None;
        }

        // Precompute screen scale to minimize operations
        let screen_x = self.screen_bounds[0] * 0.5;
        let screen_y = self.screen_bounds[1] * 0.5;

        // Single Vec2 construction at the end
        Some(Vec2::new(
            (ndc_x + 1.0) * screen_x,
            (1.0 - ndc_y) * screen_y
        ))
    }
}