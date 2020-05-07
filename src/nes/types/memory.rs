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

    pub fn is_pos(&self) -> bool {
        self.0 & 0x80 == 0x00
    }

    pub fn is_neg(&self) -> bool {
        !self.is_pos()
    }

    pub fn with_set() -> Byte {
        0xFF.into()
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

    pub fn from_bytes(lo: Byte, hi: Byte) -> Word {
        let word: Word = lo.into_lo_word() | hi.into_hi_word();
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

    pub fn into_lo(self) -> Byte {
        let byte: Byte = ((self.0 & 0x00FF) as u8).into();
        byte
    }

    pub fn into_hi(self) -> Byte {
        let byte: Byte = (((self.0 & 0xFF00) >> 8) as u8).into();
        byte
    }

    pub fn from_bytes(lo: Byte, hi: Byte) -> Addr {
        let addr: Addr = lo.into_lo_addr() | hi.into_hi_addr();
        addr
    }

    pub fn with_hi_set() -> Addr {
        0xFF00.into()
    }

    pub fn with_lo_set() -> Addr {
        0x00FF.into()
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
