// __int64 newplayerent(void)

// https://github.com/assaultcube/AC/blob/da5cb69c009b4c8fafbb2498787bd4b05d0274e7/source/src/entity.h#L59
pub const NUM_GUNS: u32 = 9;

#[derive(Copy, Clone, Debug)]
pub enum WeaponName {
    Knife = 0,
    Pistol,
    Carbine,
    Shotgun,
    Subgun,
    Sniper,
    Assault,
    Grenade,
    Akimbo,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Weapon {
    _pad_0000: [u8; 20], // 0x0000
    pub ammo_ptr: u32, // 0x0014
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Ammo {
    pub current: u32,           // 0x0000
    _pad_0004: [u8; 68], // 0x0004
    pub usage_count: u32,    // 0x0048 -- counts the number of times this weapon has been used
}