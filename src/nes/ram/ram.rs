use crate::prelude::*;

#[derive(Default)]
pub struct Ram {
    mem: Vec<Byte>,
}

impl Ram {
    pub fn with_size(size: usize) -> Self {
        Self {
            mem: vec![Byte(0); size],
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
