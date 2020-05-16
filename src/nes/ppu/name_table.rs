use crate::cartridge::Mirror;
use crate::prelude::*;

const SIZE: usize = 1024;

pub struct NameTable {
    mem_0: Vec<Byte>,
    mem_1: Vec<Byte>,
}

impl NameTable {
    pub fn new() -> Self {
        Self {
            mem_0: vec![Byte(0); SIZE],
            mem_1: vec![Byte(0); SIZE],
        }
    }

    fn normalize_addr(addr: Addr) -> Addr {
        addr & 0x0FFF.into()
    }

    pub fn read(&self, addr: Addr, mirror: Mirror) -> Byte {
        let addr = Self::normalize_addr(addr);
        let sub_addr = addr & 0x03FF.into();
        match mirror {
            Mirror::Horizontal => match addr {
                Addr(0x0000..=0x03FF) => self.mem_0[sub_addr.as_usize()],
                Addr(0x0400..=0x07FF) => self.mem_0[sub_addr.as_usize()],
                Addr(0x0800..=0x0BFF) => self.mem_1[sub_addr.as_usize()],
                Addr(0x0C00..=0x0FFF) => self.mem_1[sub_addr.as_usize()],
                _ => Byte(0),
            },
            Mirror::Vertical => match addr {
                Addr(0x0000..=0x03FF) => self.mem_0[sub_addr.as_usize()],
                Addr(0x0400..=0x07FF) => self.mem_1[sub_addr.as_usize()],
                Addr(0x0800..=0x0BFF) => self.mem_0[sub_addr.as_usize()],
                Addr(0x0C00..=0x0FFF) => self.mem_1[sub_addr.as_usize()],
                _ => Byte(0),
            },
            _ => Byte(0),
        }
    }

    pub fn write(&mut self, addr: Addr, v: Byte, mirror: Mirror) {
        let addr = Self::normalize_addr(addr);
        let sub_addr = addr & 0x03FF.into();
        match mirror {
            Mirror::Horizontal => match addr {
                Addr(0x0000..=0x03FF) => self.mem_0[sub_addr.as_usize()] = v,
                Addr(0x0400..=0x07FF) => self.mem_0[sub_addr.as_usize()] = v,
                Addr(0x0800..=0x0BFF) => self.mem_1[sub_addr.as_usize()] = v,
                Addr(0x0C00..=0x0FFF) => self.mem_1[sub_addr.as_usize()] = v,
                _ => {}
            },
            Mirror::Vertical => match addr {
                Addr(0x0000..=0x03FF) => self.mem_0[sub_addr.as_usize()] = v,
                Addr(0x0400..=0x07FF) => self.mem_1[sub_addr.as_usize()] = v,
                Addr(0x0800..=0x0BFF) => self.mem_0[sub_addr.as_usize()] = v,
                Addr(0x0C00..=0x0FFF) => self.mem_1[sub_addr.as_usize()] = v,
                _ => {}
            },
            _ => {}
        };
    }
}
