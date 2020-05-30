use crate::prelude::*;

const SIZE: usize = 2048; // 2 kb

#[derive(Default)]
pub struct Ram {
    mem: Vec<Byte>,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            mem: vec![Byte(0); SIZE],
        }
    }

    pub fn read(&self, addr: Addr) -> Byte {
        // Read RAM
        //   0b‭0001'1010'0001'0010‬ {0x1A12}
        // & 0b0000'0111'1111'1111 {0x07FF}
        // = 0b‭0000'0010'0001'0010‬ {0x0212}
        let trunc_addr = addr & Addr(0x07FF);
        self.mem[trunc_addr.as_usize()]
    }

    pub fn write(&mut self, addr: Addr, v: Byte) {
        // Write into RAM
        //   0b‭0001'1010'0001'0010‬ {0x1A12}
        // & 0b0000'0111'1111'1111 {0x07FF}
        // = 0b‭0000'0010'0001'0010‬ {0x0212}
        let trunc_addr = addr & Addr(0x07FF);
        self.mem[trunc_addr.as_usize()] = v;
    }

    pub fn size(&self) -> usize {
        self.mem.len()
    }

    pub fn dump(&self) -> &Vec<Byte> {
        &self.mem
    }
}
