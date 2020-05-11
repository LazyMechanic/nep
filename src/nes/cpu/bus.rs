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
            // Read RAM
            //   0b1000'1111'1111'1111 {0x8FFF}
            // & 0b0111'1111'1111'1111 {0x7FFF}
            // = 0b0000'1111'1111'1111 {0x0FFF}
            Addr(0x0000..=0x1FFF) => self.ram.read(addr & 0x07FF.into()),
            // Read ROM
            //   0b1000'1001'1001'0011 {0x8993}
            // - 0b1000'0000'0000'0000 {0x8000}
            // = 0b0000'1001'1001'0011 {0x0993}
            Addr(0x8000..=0xBFFF) => self.rom.read(addr - 0x8000.into()),
            // Read ROM if small cartridge (<= 16kb)
            //   0b1100'1001'1001'0011 {0xC993}
            // - 0b1100'0000'0000'0000 {0xC000}
            // = 0b0000'1001'1001'0011 {0x0993}
            Addr(0xC000..=0xFFFF) if self.rom.size() <= 16_384 /* bytes */ => {
                self.rom.read(addr - 0xC000.into())
            }
            // Read ROM in other case
            //   0b1100'1001'1001'0011 {0xC993}
            // - 0b1000'0000'0000'0000 {0x8000}
            // = 0b0100'1001'1001'0011 {0x4993}
            Addr(0xC000..=0xFFFF) => self.rom.read(addr - 0x8000.into()),
            _ => panic!("[CPUBUS] Read from an illegal address (0x{:X})", addr),
        }
    }

    fn write(&mut self, addr: Addr, v: Byte) {
        match addr {
            //   0b1000'1111'1111'1111 {0x8FFF}
            // & 0b0111'1111'1111'1111 {0x7FFF}
            // = 0b0000'1111'1111'1111 {0x0FFF}
            Addr(0x0000..=0x1FFF) => self.ram.write(addr & 0x07FF.into(), v),
            _ => panic!("[CPUBUS] Write to an illegal address (0x{:X})", addr),
        }
    }
}
