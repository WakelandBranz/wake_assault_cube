// Thank you Valthrun https://github.com/Valthrun/Valthrun/blob/aa316373a7769cb4fe6686f99f50e1596cf15128/controller/src/view/world.rs

use glam::{Vec2, Mat4};

#[derive(Default)]
/// View controller which helps resolve in game
/// coordinates into 2d screen coordinates.
pub struct ViewController {
    pub view_matrix: Mat4,
    pub screen_bounds: Vec2,
}

impl ViewController {
    pub fn update_screen_bounds(&mut self, bounds: Vec2) {
        self.screen_bounds = bounds;
    }
}


