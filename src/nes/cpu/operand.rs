use crate::types::*;

#[derive(Debug, Clone)]
pub enum Operand {
    None,
    Byte(Byte),
    Word(Word),
}

impl Operand {
    pub fn unwrap_byte(self) -> Byte {
        self.expect_byte("expected Operand::Byte but other handled")
    }

    pub fn unwrap_word(self) -> Word {
        self.expect_word("expected Operand::Word but other handled")
    }

    pub fn unwrap_none(self) {
        self.expect_none("expected Operand::None but other handled")
    }

    pub fn is_none(&self) -> bool {
        return match self {
            Operand::None => true,
            _ => false,
        };
    }

    pub fn expect_byte(self, msg: &'static str) -> Byte {
        return match self {
            Operand::Byte(v) => v,
            _ => panic!(msg),
        };
    }

    pub fn expect_word(self, msg: &'static str) -> Word {
        return match self {
            Operand::Word(v) => v,
            _ => panic!(msg),
        };
    }

    pub fn expect_none(self, msg: &'static str) {
        return match self {
            Operand::None => (),
            _ => panic!(msg),
        };
    }
}
