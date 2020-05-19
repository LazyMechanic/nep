use crate::prelude::*;
use bitfield::*;

bitfield! {
    #[derive(
            Default,
            Clone,
            Copy,
            derive_more::Display,
            derive_more::Add,
            derive_more::Sub,
            derive_more::BitAnd,
            derive_more::BitOr,
            derive_more::BitXor,
            derive_more::Mul,
            derive_more::Div,
            derive_more::Rem,
            derive_more::Shr,
            derive_more::Shl,
            derive_more::Not,
            derive_more::AddAssign,
            derive_more::SubAssign,
            derive_more::BitAndAssign,
            derive_more::BitOrAssign,
            derive_more::BitXorAssign,
            derive_more::MulAssign,
            derive_more::DivAssign,
            derive_more::RemAssign,
            derive_more::ShrAssign,
            derive_more::ShlAssign,
            derive_more::From,
            derive_more::FromStr,
            derive_more::Into,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
        )]
    pub struct JoypadState(u8);
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

impl JoypadState {
    pub fn get_and_shift(&mut self) -> Byte {
        let int_res = self.0 & 0x01;
        self.0 >>= 1;
        Byte(int_res)
    }
}

pub struct Joypad {
    state: JoypadState,
    reg:   JoypadState,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
            reg:   Default::default(),
        }
    }

    pub fn update(&mut self, state: JoypadState) {
        self.state = state;
    }

    pub fn read(&mut self) -> Byte {
        self.reg.get_and_shift()
    }

    pub fn write(&mut self, v: Byte) {
        if v & Byte(0x01) != Byte(0x00) {
            self.reg = self.state;
        }
    }
}
