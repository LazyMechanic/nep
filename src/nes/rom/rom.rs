use crate::prelude::*;

pub struct Rom {
    mem: Vec<Byte>,
}

impl Rom {
    pub fn new(buf: Vec<Byte>) -> Self {
        Self { mem: buf }
    }

    pub fn read(&self, addr: Addr) -> Byte {
        self.mem[addr.as_usize()]
    }

    pub fn size(&self) -> usize {
        self.mem.len()
    }

    pub fn dump(&self) -> &Vec<Byte> {
        &self.mem
    }
}
