// Position offsets
pub(crate) const POSITION_X: u32 = 0x2C;
pub(crate) const POSITION_Y: u32 = 0x30;
pub(crate) const POSITION_Z: u32 = 0x28;

// Head position offsets
pub(crate) const HEAD_POSITION_X: u32 = 0x4;
pub(crate) const HEAD_POSITION_Y: u32 = 0xC;
pub(crate) const HEAD_POSITION_Z: u32 = 0x8;

// Camera angles
pub(crate) const CAMERA_X: u32 = 0x34;
pub(crate) const CAMERA_Y: u32 = 0x38;
pub(crate) const VIEW_MATRIX: u32 = 0x17DFD0;

// Ammo offsets
pub(crate) const ASSAULT_RIFLE_AMMO: u32 = 0x140;
pub(crate) const SMG_AMMO: u32 = 0x138;
pub(crate) const SNIPER_AMMO: u32 = 0x13C;
pub(crate) const SHOTGUN_AMMO: u32 = 0x134;
pub(crate) const PISTOL_AMMO: u32 = 0x12C;
pub(crate) const GRENADE_AMMO: u32 = 0x144;

// Fast fire offsets
pub(crate) const FAST_FIRE_AR: u32 = 0x164;
pub(crate) const FAST_FIRE_SNIPER: u32 = 0x160;
pub(crate) const FAST_FIRE_SHOTGUN: u32 = 0x158;

// Player state offsets
pub(crate) const AUTO_SHOOT: u32 = 0x204;
pub(crate) const HEALTH: u32 = 0xEC;
pub(crate) const ARMOR: u32 = 0xF0;
pub(crate) const PLAYER_NAME: u32 = 0x205;

// Unsure if these work
pub(crate) const PRIMARY: u32 = 0x108;
pub(crate) const NEXT_PRIMARY: u32 = 0x10C;
pub(crate) const AKIMBO: u32 = 0x114;
pub(crate) const AMMO_ARRAY: u32 = 0x13C;
pub(crate) const MAG_ARRAY: u32 = 0x118;
pub(crate) const GRENADE: u32 = 0x158;
pub(crate) const GUN_INFO: u32 = 0x15C;

// Base pointer
pub const LOCAL_PLAYER: u32 = 0x0017E0A8;