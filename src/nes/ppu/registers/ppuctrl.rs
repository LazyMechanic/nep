use crate::prelude::*;
use bitfield::*;

bitfield! {
    #[derive(Default, Clone, Copy)]
    pub struct PpuCtrl(u8);
    impl Debug;
    // =============================================== Bit position: 7654 3210
    pub bool, enable_nmi,         set_enable_nmi:         0;    // 0b0000_000X
    pub bool, slave_mode,         set_slave_mode:         1;    // 0b0000_00X0
    pub bool, sprite_size,        set_sprite_size:        2;    // 0b0000_0X00
    pub bool, pattern_background, set_pattern_background: 3;    // 0b0000_X000
    pub bool, pattern_sprite,     set_pattern_sprite:     4;    // 0b000X_0000
    pub bool, increment_mode,     set_increment_mode:     5;    // 0b00X0_0000
    pub u8,   nametable,          set_nametable:          7, 6; // 0bXX00_0000
}

impl From<u8> for PpuCtrl {
    fn from(v: u8) -> Self {
        Self(v)
    }
}

impl From<PpuCtrl> for u8 {
    fn from(v: PpuCtrl) -> Self {
        v.0
    }
}

impl From<Byte> for PpuCtrl {
    fn from(v: Byte) -> Self {
        Self(v.into())
    }
}

impl From<PpuCtrl> for Byte {
    fn from(v: PpuCtrl) -> Self {
        Self(v.0)
    }
}
