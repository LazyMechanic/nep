use crate::prelude::*;
use crate::ram::Ram;

#[derive(Debug, Clone, Copy)]
pub struct Oam {
    addr: Addr,
}

impl Oam {
    pub fn new() -> Self {
        Self {
            addr: 0x0000.into(),
        }
    }

    pub fn reset(&mut self) {
        self.addr = 0x0000.into();
    }

    pub fn addr(&self) -> Addr {
        self.addr
    }

    // OAM address ($2003) > write
    // Common name: OAMADDR
    // Description: OAM address port
    // Access: write
    // Write the address of OAM you want to access here.
    // Most games just write $00 here and then use OAMDMA.
    // (DMA is implemented in the 2A03/7 chip and works by repeatedly writing to OAMDATA)
    pub fn write_addr(&mut self, v: Byte) {
        self.addr = v.as_lo_addr();
    }

    // OAM data ($2004) <> read/write
    // Common name: OAMDATA
    // Description: OAM data port
    // Access: read, write
    // Write OAM data here.
    // Writes will increment OAMADDR after the write;
    // reads during vertical or forced blanking return the value from OAM at that address but do not increment.
    pub fn write_data(&mut self, ram: &mut Ram, v: Byte) {
        ram.write(self.addr, v);
        self.addr.inc();
    }

    pub fn read_data(&self, ram: &Ram) -> Byte {
        ram.read(self.addr)
    }
}
