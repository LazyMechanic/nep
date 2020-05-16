use crate::prelude::*;
use crate::ram::Ram;

const SIZE: usize = 0x20;

#[derive(Debug)]
pub enum PaletteType {
    Sprite,
    Background,
}

#[derive(Default)]
pub struct PaletteRam {
    mem: Vec<Byte>,
}

impl PaletteRam {
    pub fn new() -> Self {
        Self {
            mem: vec![Byte(0); SIZE],
        }
    }

    fn normalize_addr(addr: Addr) -> Addr {
        let addr = addr & 0x001F.into();
        return match addr {
            Addr(0x0010) => Addr(0x0000),
            Addr(0x0014) => Addr(0x0004),
            Addr(0x0018) => Addr(0x0008),
            Addr(0x001C) => Addr(0x000C),
            _ => addr,
        };
    }

    pub fn read(&self, addr: Addr) -> Byte {
        let addr = Self::normalize_addr(addr);
        self.mem[addr.as_usize()]
    }

    pub fn write(&mut self, addr: Addr, v: Byte) {
        let addr = Self::normalize_addr(addr);
        self.mem[addr.as_usize()] = v
    }
}
