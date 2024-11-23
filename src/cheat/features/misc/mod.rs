use serde::{Deserialize, Serialize};
use crate::cheat::features::misc::fps::FPS;

pub(crate) mod fps;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Misc {
    pub fps: FPS,
}