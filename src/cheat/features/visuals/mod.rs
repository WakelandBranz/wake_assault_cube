use serde::{Deserialize, Serialize};
use box_esp::BoxEsp;
use name_esp::NameEsp;
use healthbar_esp::HealthbarESP;
use head_esp::HeadEsp;

pub(crate) mod render_context;
pub(crate) mod box_esp;
pub(crate) mod head_esp;
pub(crate) mod name_esp;
pub(crate) mod healthbar_esp;


#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Visuals {
    pub box_esp: BoxEsp,
    pub head_esp: HeadEsp,
    pub name_esp: NameEsp,
    pub healthbar_esp: HealthbarESP,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub(crate) enum Position {
    Top,
    TopLeft,
    TopRight,
    Bottom,
    BottomLeft,
    BottomRight,
    Left,
    Right
}
