use serde::{Deserialize, Serialize};
use box_esp::BoxEsp;
use name_esp::NameEsp;
use healthbar_esp::HealthbarESP;

pub(crate) mod render_context;
pub(crate) mod box_esp;
pub(crate) mod name_esp;
pub(crate) mod healthbar_esp;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Visuals {
    pub box_esp: BoxEsp,
    pub name_esp: NameEsp,
    pub healthbar_esp: HealthbarESP,
}

