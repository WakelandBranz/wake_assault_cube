const GUN_INFO_SIZE: u32 = 0x68;
const GUN_INFO_ARRAY: u32 = 0x5EA0C0;

const TITLE: u32 = 23;
const SOUND: u32 = 23 + 42;
const RELOAD: u32 = SOUND + 0x2;
const RELOAD_TIME: u32 = RELOAD + 0x2;
const ATTACK_DELAY: u32 = RELOAD_TIME + 0x2;
const DAMAGE: u32 = ATTACK_DELAY + 0x2;
const PIERCING: u32 = DAMAGE + 0x2;
const PROJ_SPEED: u32 = PIERCING + 0x2;
const PART: u32 = PROJ_SPEED + 0x2;
const SPREAD: u32 = PART + 0x2;
const RECOIL: u32 = SPREAD + 0x2;
const CURRENT_AMMO: u32 = 0x35A + 0x10;