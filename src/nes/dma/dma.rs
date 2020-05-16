use crate::cpu::bus::Bus;
use crate::prelude::*;
use std::borrow::BorrowMut;

#[derive(Debug)]
pub struct Dma {
    reg:        Byte,
    should_run: bool,
}

impl Dma {
    pub fn new() -> Self {
        Self {
            reg:        Byte(0),
            should_run: false,
        }
    }

    pub fn write(&mut self, v: Byte) {
        self.reg = v;
        self.should_run = true;
    }

    pub fn should_run(&self) -> bool {
        self.should_run
    }

    pub fn step(&mut self, cpu_bus: &mut Bus) {
        let addr = self.reg.as_hi_addr();
        for i in 0..0x0100 {
            let v = cpu_bus.read(addr + i.into());
        }
        self.should_run = false;
    }
}
