use crate::prelude::*;
use crate::ram::Ram;
use crate::rom::Rom;

pub trait CpuBus {
    fn read(&mut self, addr: Addr) -> Byte;
    fn write(&mut self, addr: Addr, v: Byte);
}

pub struct Bus<'a> {
    rom: &'a Rom,
    ram: &'a mut Ram,
}

impl<'a> Bus<'a> {
    pub fn new(rom: &'a Rom, ram: &'a mut Ram) -> Self {
        Self { rom, ram }
    }
}

impl<'a> CpuBus for Bus<'a> {
    fn read(&mut self, addr: Addr) -> Byte {
        match addr {
            Addr(0x0000..=0x1FFF) => self.ram.read(addr & 0x07FF.into()),
            Addr(0x8000..=0xBFFF) => self.rom.read(addr - 0x8000.into()),
            Addr(0xC000..=0xFFFF) if self.rom.size() <= 16384 /* bytes */ => {
                // If small cartridge (<= 16kb)
                self.rom.read(addr - 0xC000.into())
            }
            Addr(0xC000..=0xFFFF) => self.rom.read(addr - 0x8000.into()),
            _ => panic!("[CPUBUS] Read from an illegal address (0x{:X})", addr),
        }
    }

    fn write(&mut self, addr: Addr, v: Byte) {
        unimplemented!()
    }
}
