use crate::prelude::*;

const SIZE: usize = 4096;

pub struct PatternTable {
    mem_0: Vec<Byte>,
    mem_1: Vec<Byte>,
}

impl PatternTable {
    pub fn new() -> Self {
        Self {
            mem_0: vec![Byte(0); SIZE],
            mem_1: vec![Byte(0); SIZE],
        }
    }

    pub fn read(&self, addr: Addr) -> Byte {
        // If the cartridge cant map the address, have
        // a physical location ready here
        const BIT_NO: u16 = 11;
        let sub_addr = addr & 0x0FFF.into();
        match addr.inspect_bit(BIT_NO) {
            false => self.mem_0[sub_addr.as_usize()],
            true => self.mem_1[sub_addr.as_usize()],
        }
    }

    pub fn write(&mut self, addr: Addr, v: Byte) {
        // If the cartridge cant map the address, have
        // a physical location ready here
        const BIT_NO: u16 = 11;
        let sub_addr = addr & 0x0FFF.into();
        match addr.inspect_bit(BIT_NO) {
            false => self.mem_0[sub_addr.as_usize()] = v,
            true => self.mem_1[sub_addr.as_usize()] = v,
        }
    }
}
