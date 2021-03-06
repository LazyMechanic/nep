use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    None,
    Byte(Byte),
    Addr(Addr),
}

impl Operand {
    pub fn unwrap_none(self) {
        self.expect_none("expected Operand::None but other handled")
    }

    pub fn unwrap_byte(self) -> Byte {
        self.expect_byte("expected Operand::Byte but other handled")
    }

    pub fn unwrap_addr(self) -> Addr {
        self.expect_addr("expected Operand::Addr but other handled")
    }

    pub fn is_none(&self) -> bool {
        return match self {
            Operand::None => true,
            _ => false,
        };
    }

    pub fn expect_none(self, msg: &'static str) {
        return match self {
            Operand::None => (),
            _ => panic!(msg),
        };
    }

    pub fn expect_byte(self, msg: &'static str) -> Byte {
        return match self {
            Operand::Byte(v) => v,
            _ => panic!(msg),
        };
    }

    pub fn expect_addr(self, msg: &'static str) -> Addr {
        return match self {
            Operand::Addr(v) => v,
            _ => panic!(msg),
        };
    }
}
