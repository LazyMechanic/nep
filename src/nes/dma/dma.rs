use crate::nes::ppu::oam::Oam;
use crate::nes::ram::Ram;
use crate::prelude::*;

pub struct Dma {
    has_request: bool,
    wait_start:  bool,
    data:        Byte,
    page:        Addr,
    addr:        Addr,
    cycles:      u16,
}

impl Dma {
    pub fn new() -> Self {
        Self {
            has_request: false,
            wait_start:  true,
            data:        Byte(0x00),
            page:        Addr(0x0000),
            addr:        Addr(0x0000),
            cycles:      0,
        }
    }

    pub fn has_request(&self) -> bool {
        self.has_request
    }

    pub fn wait_start(&self) -> bool {
        self.wait_start
    }

    pub fn start(&mut self) {
        self.wait_start = false;
    }

    fn reset(&mut self) {
        self.has_request = false;
        self.wait_start = true;
        self.addr = Addr(0x0000);
        self.page = Addr(0x0000);
        self.data = Byte(0x00);
        self.cycles = 0;
    }

    fn need_read(&self) -> bool {
        self.cycles % 2 == 0
    }

    pub fn step(&mut self, ram: &mut Ram, oam: &mut Oam) {
        if self.need_read() {
            // Read from RAM
            let addr = self.page | self.addr;
            self.data = ram.read(addr);
            self.cycles += 1;
        } else {
            // Write to OAM
            // On odd clock cycles, write to PPU OAM
            oam.write(self.addr, self.data);
            self.addr.inc();
            self.cycles += 1;

            if self.addr == Addr(0x0000) {
                self.reset();
            }
        }
    }

    pub fn write(&mut self, v: Byte) {
        self.reset();
        self.page = v.as_hi_addr();
        self.has_request = true;
    }
}
