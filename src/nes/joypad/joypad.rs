use crate::prelude::*;
use bitfield;

bitfield! {
    #[derive(Default, Clone, Copy)]
    pub struct Joypad(u8);
    impl Debug;
    // ==================== Bit position: 7654 3210
    pub bool, right,  set_right:  0; // 0b0000_000*
    pub bool, left,   set_left:   1; // 0b0000_00*0
    pub bool, down,   set_down:   2; // 0b0000_0*00
    pub bool, up,     set_up:     3; // 0b0000_*000
    pub bool, start,  set_start:  4; // 0b000*_0000
    pub bool, select, set_select: 5; // 0b00*0_0000
    pub bool, b,      set_b:      6; // 0b0*00_0000
    pub bool, a,      set_a:      7; // 0b*000_0000
}

impl PpuMask {
    pub fn new() -> Self {
        Self(0)
    }
}

impl From<u8> for Joypad {
    fn from(v: u8) -> Self {
        Self(v)
    }
}

impl From<Joypad> for u8 {
    fn from(v: Joypad) -> Self {
        v.0
    }
}

impl From<Byte> for Joypad {
    fn from(v: Byte) -> Self {
        Self(v.into())
    }
}

impl From<Joypad> for Byte {
    fn from(v: Joypad) -> Self {
        Self(v.0)
    }
}
