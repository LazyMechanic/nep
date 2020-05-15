use crate::prelude::*;
use bitfield::*;

bitfield! {
    #[derive(Default, Clone, Copy)]
    pub struct PpuCtrl(u8);
    impl Debug;
    // ============================================ Bit position: 7654 3210
    pub bool, nametable_x,        set_nametable_x:        0; // 0b0000_000*
    pub bool, nametable_y,        set_nametable_y:        1; // 0b0000_00*0
    pub bool, increment_mode,     set_increment_mode:     2; // 0b0000_0*00
    pub bool, pattern_sprite,     set_pattern_sprite:     3; // 0b0000_*000
    pub bool, pattern_background, set_pattern_background: 4; // 0b000*_0000
    pub bool, sprite_size,        set_sprite_size:        5; // 0b00*0_0000
    pub bool, slave_mode,         set_slave_mode:         6; // 0b0*00_0000
    pub bool, enable_nmi,         set_enable_nmi:         7; // 0b*000_0000
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
