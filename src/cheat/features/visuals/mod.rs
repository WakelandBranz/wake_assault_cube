use serde::{Deserialize, Serialize};
use box_esp::BoxEsp;
use name_esp::NameEsp;

pub(crate) mod box_esp;
pub(crate) mod name_esp;
pub(crate) mod render_context;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Visuals {
    pub box_esp: BoxEsp,
    pub name_esp: NameEsp,
}

fn get_color_rgba(color: [f32; 4]) -> (u8, u8, u8, u8) {
    (
        (color[0] * 255.0) as u8,
        (color[1] * 255.0) as u8,
        (color[2] * 255.0) as u8,
        (color[3] * 255.0) as u8,
    )
}