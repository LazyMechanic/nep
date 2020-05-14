use super::mapper::Mapper;
use crate::prelude::*;

pub struct Mapper000 {
    prg_banks: usize,
    chr_banks: usize,
}

impl Mapper for Mapper000 {
    // if PRGROM is 16KB
    //     CPU Address Bus          PRG ROM
    //     0x8000 -> 0xBFFF: Map    0x0000 -> 0x3FFF
    //     0xC000 -> 0xFFFF: Mirror 0x0000 -> 0x3FFF
    // if PRGROM is 32KB
    //     CPU Address Bus          PRG ROM
    //     0x8000 -> 0xFFFF: Map    0x0000 -> 0x7FFF
    fn map_read(&mut self, addr: Addr, mapped_addr: &mut ExtAddr, v: &mut Byte) -> bool {
        match addr {
            Addr(0x8000..=0xFFFF) => {
                *mapped_addr = if self.prg_banks > 1 {
                    (addr & 0x7FFF.into()).as_lo_ext_addr()
                } else {
                    (addr & 0x3FFF.into()).as_lo_ext_addr()
                };
                true
            }
            //Addr(0x6000..=0x7FFF) => (addr.as_lo_ext_addr(), Byte(0)),
            _ => false,
        }
    }

    // if PRGROM is 16KB
    //     CPU Address Bus          PRG ROM
    //     0x8000 -> 0xBFFF: Map    0x0000 -> 0x3FFF
    //     0xC000 -> 0xFFFF: Mirror 0x0000 -> 0x3FFF
    // if PRGROM is 32KB
    //     CPU Address Bus          PRG ROM
    //     0x8000 -> 0xFFFF: Map    0x0000 -> 0x7FFF
    fn map_write(&mut self, addr: Addr, mapped_addr: &mut ExtAddr, v: Byte) -> bool {
        match addr {
            Addr(0x8000..=0xFFFF) => {
                *mapped_addr = if self.prg_banks > 1 {
                    (addr & 0x7FFF.into()).as_lo_ext_addr()
                } else {
                    (addr & 0x3FFF.into()).as_lo_ext_addr()
                };
                true
            }
            //Addr(0x6000..=0x7FFF) => (addr.as_lo_ext_addr(), Byte(0)),
            _ => false,
        }
    }

    // There is no mapping required for PPU
    // PPU Address Bus          CHR ROM
    // 0x0000 -> 0x1FFF: Map    0x0000 -> 0x1FFF
    fn map_read_chr(&mut self, addr: Addr, mapped_addr: &mut ExtAddr) -> bool {
        match addr {
            Addr(0x0000..=0x1FFF) => {
                *mapped_addr = addr.as_lo_ext_addr();
                true
            }
            _ => false,
        }
    }

    // There is no mapping required for PPU
    // PPU Address Bus          CHR ROM
    // 0x0000 -> 0x1FFF: Map    0x0000 -> 0x1FFF
    fn map_write_chr(&mut self, addr: Addr, mapped_addr: &mut ExtAddr) -> bool {
        match addr {
            // Treat as RAM
            Addr(0x0000..=0x1FFF) if self.chr_banks == 0 => {
                *mapped_addr = addr.as_lo_ext_addr();
                true
            }
            _ => false,
        }
    }
}

impl Mapper000 {
    pub fn new(prg_banks: usize, chr_banks: usize) -> Self {
        Self {
            prg_banks,
            chr_banks,
        }
    }
}
