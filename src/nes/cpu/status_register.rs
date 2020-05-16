use crate::prelude::*;
use bitfield::*;

bitfield! {
    #[derive(Clone, Copy)]
    pub struct StatusRegister(u8);
    impl Debug;
    // =============================== Bit position: 7654 3210
    pub bool, carry,        set_carry:         0; // 0000_0001
    pub bool, zero,         set_zero:          1; // 0000_0010
    pub bool, interrupt,    set_interrupt:     2; // 0000_0100
    pub bool, decimal_mode, set_decimal_mode:  3; // 0000_1000
    pub bool, break_mode,   set_break_mode:    4; // 0001_0000
    pub bool, reserved,     set_reserved:      5; // 0010_0000
    pub bool, overflow,     set_overflow:      6; // 0100_0000
    pub bool, negative,     set_negative:      7; // 1000_0000
}

impl StatusRegister {
    pub fn new() -> Self {
        let mut s = Self(0);
        s.set_reserved(true);
        s
    }
}

impl From<u8> for StatusRegister {
    fn from(v: u8) -> Self {
        Self(v)
    }
}

impl From<StatusRegister> for u8 {
    fn from(v: StatusRegister) -> Self {
        v.0
    }
}

impl From<Byte> for StatusRegister {
    fn from(v: Byte) -> Self {
        Self(v.into())
    }
}

impl From<StatusRegister> for Byte {
    fn from(v: StatusRegister) -> Self {
        Byte(v.0)
    }
}
