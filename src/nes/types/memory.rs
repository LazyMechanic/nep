use super::macros::*;

math_type!(Byte, u8);
math_type!(Word, u16);
math_type!(Addr, u16);
math_type!(ExtAddr, u32);

impl Byte {
    pub fn as_hi_word(&self) -> Word {
        let word: Word = (((self.0 as u16) << 8) & 0xFF00).into();
        word
    }

    pub fn as_lo_word(&self) -> Word {
        let word: Word = ((self.0 as u16) & 0x00FF).into();
        word
    }

    pub fn as_hi_addr(&self) -> Addr {
        let addr: Addr = (((self.0 as u16) << 8) & 0xFF00).into();
        addr
    }

    pub fn as_lo_addr(&self) -> Addr {
        let addr: Addr = ((self.0 as u16) & 0x00FF).into();
        addr
    }

    pub fn is_pos(&self) -> bool {
        self.0 & 0x80 == 0x00
    }

    pub fn is_neg(&self) -> bool {
        !self.is_pos()
    }

    pub fn with_set() -> Byte {
        0xFF.into()
    }

    pub fn is_set(&self) -> bool {
        self.0 == 0xFF
    }

    pub fn is_clear(&self) -> bool {
        self.0 == 0x00
    }

    pub fn inspect_bit(&self, bit_no: u8) -> bool {
        match bit_no {
            0..=7 => self.0 & (1u8 << bit_no) != 0,
            _ => false,
        }
    }
}

impl From<Byte> for i16 {
    fn from(v: Byte) -> Self {
        v.0 as i16
    }
}

impl Word {
    pub fn lo(&self) -> Byte {
        let byte: Byte = ((self.0 & 0x00FF) as u8).into();
        byte
    }

    pub fn hi(&self) -> Byte {
        let byte: Byte = (((self.0 & 0xFF00) >> 8) as u8).into();
        byte
    }

    pub fn lo_word(&self) -> Word {
        let v: Word = *self & 0x00FF.into();
        v
    }

    pub fn hi_word(&self) -> Word {
        let v: Word = *self & 0xFF00.into();
        v
    }

    pub fn inspect_bit(&self, bit_no: u16) -> bool {
        match bit_no {
            0..=15 => self.0 & (1u16 << bit_no) != 0,
            _ => false,
        }
    }

    pub fn set_lo(&mut self, v: Byte) {
        *self = (*self & Word::with_hi_set()) | v.as_lo_word();
    }

    pub fn set_hi(&mut self, v: Byte) {
        *self = (*self & Word::with_lo_set()) | v.as_hi_word();
    }

    pub fn from_bytes(lo: Byte, hi: Byte) -> Word {
        let word: Word = lo.as_lo_word() | hi.as_hi_word();
        word
    }

    pub fn with_hi_set() -> Word {
        0xFF00.into()
    }

    pub fn with_lo_set() -> Word {
        0x00FF.into()
    }
}

impl Addr {
    pub fn lo(&self) -> Byte {
        let byte: Byte = ((self.0 & 0x00FF) as u8).into();
        byte
    }

    pub fn hi(&self) -> Byte {
        let byte: Byte = (((self.0 & 0xFF00) >> 8) as u8).into();
        byte
    }

    pub fn lo_addr(&self) -> Addr {
        let v: Addr = *self & Addr(0x00FF);
        v
    }

    pub fn hi_addr(&self) -> Addr {
        let v: Addr = *self & Addr(0xFF00);
        v
    }

    pub fn as_lo_ext_addr(&self) -> ExtAddr {
        let v: ExtAddr = (0x0000FFFF & ((self.0 as u32) << 0)).into();
        v
    }

    pub fn as_hi_ext_addr(&self) -> ExtAddr {
        let v: ExtAddr = (0xFFFF0000 & ((self.0 as u32) << 16)).into();
        v
    }

    pub fn set_lo(&mut self, v: Byte) {
        *self = (*self & Addr::with_hi_set()) | v.as_lo_addr();
    }

    pub fn set_hi(&mut self, v: Byte) {
        *self = (*self & Addr::with_lo_set()) | v.as_hi_addr();
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn from_bytes(lo: Byte, hi: Byte) -> Addr {
        let addr: Addr = lo.as_lo_addr() | hi.as_hi_addr();
        addr
    }

    pub fn with_hi_set() -> Addr {
        Addr(0xFF00)
    }

    pub fn with_lo_set() -> Addr {
        Addr(0x00FF)
    }

    pub fn inspect_bit(&self, bit_no: u16) -> bool {
        match bit_no {
            0..=15 => self.0 & (1u16 << bit_no) != 0,
            _ => false,
        }
    }
}

impl ExtAddr {
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl From<Addr> for i16 {
    fn from(v: Addr) -> Self {
        v.0 as i16
    }
}

impl From<i16> for Addr {
    fn from(v: i16) -> Self {
        Self(v as u16)
    }
}

impl From<Word> for Addr {
    fn from(v: Word) -> Self {
        Self(v.0)
    }
}

impl From<&Word> for Addr {
    fn from(v: &Word) -> Self {
        Self(v.0)
    }
}

impl From<&mut Word> for Addr {
    fn from(v: &mut Word) -> Self {
        Self(v.0)
    }
}

impl From<Addr> for Word {
    fn from(v: Addr) -> Self {
        Self(v.0)
    }
}

impl From<&Addr> for Word {
    fn from(v: &Addr) -> Self {
        Self(v.0)
    }
}

impl From<&mut Addr> for Word {
    fn from(v: &mut Addr) -> Self {
        Self(v.0)
    }
}

impl From<Addr> for ExtAddr {
    fn from(v: Addr) -> Self {
        Self(v.0 as u32)
    }
}

impl From<&Addr> for ExtAddr {
    fn from(v: &Addr) -> Self {
        Self(v.0 as u32)
    }
}

impl From<&mut Addr> for ExtAddr {
    fn from(v: &mut Addr) -> Self {
        Self(v.0 as u32)
    }
}
