use super::macros::*;

math_type!(Byte, u8);
math_type!(Word, u16);
math_type!(Addr, u16);

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

    pub fn into_hi_word(self) -> Word {
        let word: Word = (((self.0 as u16) << 8) & 0xFF00).into();
        word
    }

    pub fn into_lo_word(self) -> Word {
        let word: Word = ((self.0 as u16) & 0x00FF).into();
        word
    }

    pub fn into_hi_addr(self) -> Addr {
        let addr: Addr = (((self.0 as u16) << 8) & 0xFF00).into();
        addr
    }

    pub fn into_lo_addr(self) -> Addr {
        let addr: Addr = ((self.0 as u16) & 0x00FF).into();
        addr
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

    pub fn into_lo(self) -> Byte {
        let byte: Byte = ((self.0 & 0x00FF) as u8).into();
        byte
    }

    pub fn into_hi(self) -> Byte {
        let byte: Byte = (((self.0 & 0xFF00) >> 8) as u8).into();
        byte
    }
}

impl From<Word> for Addr {
    fn from(v: Word) -> Self {
        Self(v.0)
    }
}

impl From<Addr> for Word {
    fn from(v: Addr) -> Self {
        Self(v.0)
    }
}
