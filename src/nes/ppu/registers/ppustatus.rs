use crate::prelude::*;
use bitfield::*;

bitfield! {
    #[derive(Default, Clone, Copy)]
    pub struct PpuStatus(u8);
    impl Debug;
    // ====================================== Bit position: 7654 3210
    // UNUSED                                          // 0b000*_****
    pub bool, vertical_blank,  set_vertical_blank:  5; // 0b00*0_0000
    pub bool, sprite_zero_hit, set_sprite_zero_hit: 6; // 0b0*00_0000
    pub bool, sprite_overflow, set_sprite_overflow: 7; // 0b*000_0000
}

impl PpuStatus {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn enable_vertical_blank(&mut self) {
        self.set_vertical_blank(true);
    }

    pub fn enable_sprite_zero_hit(&mut self) {
        self.set_sprite_zero_hit(true);
    }

    pub fn enable_sprite_overflow(&mut self) {
        self.set_sprite_overflow(true);
    }

    pub fn disable_vertical_blank(&mut self) {
        self.set_vertical_blank(false);
    }

    pub fn disable_sprite_zero_hit(&mut self) {
        self.set_sprite_zero_hit(false);
    }

    pub fn disable_sprite_overflow(&mut self) {
        self.set_sprite_overflow(false);
    }
}

impl From<u8> for PpuStatus {
    fn from(v: u8) -> Self {
        Self(v)
    }
}

impl From<PpuStatus> for u8 {
    fn from(v: PpuStatus) -> Self {
        v.0
    }
}

impl From<Byte> for PpuStatus {
    fn from(v: Byte) -> Self {
        Self(v.into())
    }
}

impl From<PpuStatus> for Byte {
    fn from(v: PpuStatus) -> Self {
        Self(v.0)
    }
}
