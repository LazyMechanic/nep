use super::Ram;
use crate::prelude::*;

const SIZE: usize = 8192; // 8 kb

#[derive(Default)]
pub struct InternalRam {
    mem: Vec<Byte>,
}

impl InternalRam {
    pub fn new() -> Self {
        Self {
            mem: vec![Byte(0); SIZE],
        }
    }
}

impl Ram for InternalRam {
    fn read(&self, addr: Addr) -> Byte {
        // Read RAM
        //   0b‭0001'1010'0001'0010‬ {0x1A12}
        // & 0b0000'0111'1111'1111 {0x07FF}
        // = 0b‭0000'0010'0001'0010‬ {0x0212}
        let trunc_addr = addr & 0x07FF.into();
        self.mem[trunc_addr.as_usize()]
    }

    fn write(&mut self, addr: Addr, v: Byte) {
        // Write into RAM
        //   0b‭0001'1010'0001'0010‬ {0x1A12}
        // & 0b0000'0111'1111'1111 {0x07FF}
        // = 0b‭0000'0010'0001'0010‬ {0x0212}
        let trunc_addr = addr & 0x07FF.into();
        self.mem[trunc_addr.as_usize()] = v;
    }

    fn size(&self) -> usize {
        self.mem.len()
    }

    fn dump(&self) -> &Vec<Byte> {
        &self.mem
    }
}
