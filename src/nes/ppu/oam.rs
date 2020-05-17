use crate::prelude::*;

const SIZE: usize = 256;

#[derive(Debug, Clone, Copy)]
pub struct OamEntry {
    pub y:    Byte, // Y position of sprite
    pub id:   Byte, // ID of tile from pattern memory
    pub attr: Byte, // Flags define how sprite should be rendered
    pub x:    Byte, // X position of sprite
}

impl OamEntry {
    pub fn new() -> Self {
        Self {
            y:    Byte(0),
            id:   Byte(0),
            attr: Byte(0),
            x:    Byte(0),
        }
    }

    pub fn from_bytes(v: (Byte, Byte, Byte, Byte)) -> Self {
        Self {
            y:    v.0,
            id:   v.1,
            attr: v.2,
            x:    v.3,
        }
    }

    pub fn into_bytes(self) -> (Byte, Byte, Byte, Byte) {
        (self.y, self.id, self.attr, self.x)
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

    pub fn read_entry(&self, entry_ind: Addr) -> OamEntry {
        let start = entry_ind.as_usize() * 4;
        let y = self.mem[start + 0];
        let id = self.mem[start + 1];
        let attr = self.mem[start + 2];
        let x = self.mem[start + 3];

        OamEntry::from_bytes((y, id, attr, x))
    }

    pub fn write_entry(&mut self, entry_ind: Addr, entry: OamEntry) {
        let start = entry_ind.as_usize();
        let v = entry.into_bytes();

        self.mem[start + 0] = v.0;
        self.mem[start + 1] = v.1;
        self.mem[start + 2] = v.2;
        self.mem[start + 3] = v.3;
    }

    pub fn size(&self) -> usize {
        self.mem.len()
    }

    pub fn dump(&self) -> &Vec<Byte> {
        &self.mem
    }
}
