use super::macros::*;

math_type!(Data, u8);
math_type!(Word, u16);
math_type!(Addr, u16);

impl From<Data> for Word {
    fn from(v: Data) -> Self { Self(v.0 as u16) }
}

impl From<Data> for Addr {
    fn from(v: Data) -> Self { Self(v.0 as u16) }
}

impl From<Word> for Addr {
    fn from(v: Word) -> Self { Self(v.0) }
}

impl From<Addr> for Word {
    fn from(v: Addr) -> Self { Self(v.0) }
}
