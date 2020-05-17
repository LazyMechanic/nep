use crate::types::*;

pub trait BoolExt {
    fn as_byte(&self) -> Byte;
    fn as_word(&self) -> Word;
    fn as_addr(&self) -> Addr;
}

impl BoolExt for bool {
    fn as_byte(&self) -> Byte {
        if *self {
            Byte(1)
        } else {
            Byte(0)
        }
    }

    fn as_word(&self) -> Word {
        if *self {
            Word(1)
        } else {
            Word(0)
        }
    }

    fn as_addr(&self) -> Addr {
        if *self {
            Addr(1)
        } else {
            Addr(0)
        }
    }
}

impl From<bool> for Addr {
    fn from(v: bool) -> Self {
        v.as_addr()
    }
}
