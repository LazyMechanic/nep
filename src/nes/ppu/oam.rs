use crate::prelude::*;

const SIZE: usize = 256;

pub struct OamEntry {
    pub y:    Byte, // Y position of sprite
    pub id:   Byte, // ID of tile from pattern memory
    pub attr: Byte, // Flags define how sprite should be rendered
    pub x:    Byte, // X position of sprite
}

impl OamEntry {
    pub fn from_bytes(v: &[Byte; 4]) -> Self {
        Self {
            y:    v[0],
            id:   v[1],
            attr: v[2],
            x:    v[3],
        }
    }
}

pub struct Oam {
    mem: Vec<Byte>,
}

impl Oam {
    pub fn new() -> Self {
        Self {
            mem: vec![Byte(0); SIZE],
        }
    }

    pub fn read(&self, addr: Addr) -> Byte {
        self.mem[addr.as_usize()]
    }

    pub fn write(&mut self, addr: Addr, v: Byte) {
        self.mem[addr.as_usize()] = v;
    }

    pub fn size(&self) -> usize {
        self.mem.len()
    }

    pub fn dump(&self) -> &Vec<Byte> {
        &self.mem
    }
}
