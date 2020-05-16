use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Part {
    Hi,
    Lo,
}

#[derive(Debug, Clone, Copy)]
pub struct PpuAddr {
    addr: Addr,
    part: Part,
}

impl PpuAddr {
    pub fn new() -> Self {
        Self {
            addr: 0x0000.into(),
            part: Part::Hi,
        }
    }

    pub fn set_part(&mut self, part: Part) {
        self.part = part;
    }

    pub fn set_part_hi(&mut self) {
        self.set_part(Part::Hi);
    }

    pub fn set_part_lo(&mut self) {
        self.set_part(Part::Lo);
    }

    pub fn addr(&self) -> Addr {
        self.addr
    }

    pub fn update(&mut self, offset: Addr) {
        self.addr += offset;
    }

    pub fn write(&mut self, v: Byte) {
        match self.part {
            Part::Hi => {
                self.addr.set_hi(v);
                self.part = Part::Lo;
            }
            Part::Lo => {
                self.addr.set_lo(v);
                self.part = Part::Hi;
            }
        }
    }
}
