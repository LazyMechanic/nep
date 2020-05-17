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
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
        )]
    pub struct AddrReg(u16);
    impl Debug;
    // ================================================================= 1111 11
    // =================================================== Bit position: 5432 1098 7654 3210
    pub u16,  from into Addr, coarse_x,    set_coarse_x:    4, 0;   // 0b0000_0000_000*_****
    pub u16,  from into Addr, coarse_y,    set_coarse_y:    9, 5;   // 0b0000_00**_***0_0000
    pub bool, into Addr,      nametable_x, set_nametable_x: 10;     // 0b0000_0*00_0000_0000
    pub bool, into Addr,      nametable_y, set_nametable_y: 11;     // 0b0000_*000_0000_0000
    pub u16,  from into Addr, fine_y,      set_fine_y:      14, 12; // 0b0***_0000_0000_0000
}

impl AddrReg {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn inc_coarse_x(&mut self) {
        self.set_coarse_x(self.coarse_x().inc());
    }

    pub fn inc_coarse_y(&mut self) {
        self.set_coarse_y(self.coarse_y().inc());
    }

    pub fn inc_fine_y(&mut self) {
        self.set_fine_y(self.fine_y().inc());
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
