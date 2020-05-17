use crate::prelude::*;
use bitfield::*;

bitfield! {
    #[derive(Default, Clone, Copy)]
    pub struct AddrReg(u16);
    impl Debug;
    // ================================================ 1111 11
    // ================================== Bit position: 5432 1098 7654 3210
    pub u16, coarse_x,    set_coarse_x:    4, 0;   // 0b0000_0000_000*_****
    pub u16, coarse_y,    set_coarse_y:    9, 5;   // 0b0000_00**_***0_0000
    pub u16, nametable_x, set_nametable_x: 10;     // 0b0000_0*00_0000_0000
    pub u16, nametable_y, set_nametable_y: 11;     // 0b0000_*000_0000_0000
    pub u16, fine_y,      set_fine_y:      12, 14; // 0b0***_0000_0000_0000
}

impl AddrReg {
    pub fn new() -> Self {
        Self(0)
    }
}

impl From<u16> for AddrReg {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl From<AddrReg> for u16 {
    fn from(v: AddrReg) -> Self {
        v.0
    }
}

impl From<Addr> for AddrReg {
    fn from(v: Addr) -> Self {
        Self(v.into())
    }
}

impl From<AddrReg> for Addr {
    fn from(v: AddrReg) -> Self {
        Self(v.0)
    }
}
