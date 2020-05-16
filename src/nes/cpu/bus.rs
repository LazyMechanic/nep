use crate::cartridge::Cartridge;
use crate::dma::Dma;
use crate::nes::ppu::Ppu;
use crate::prelude::*;
use crate::ram::Ram;

use std::cell::RefCell;
use std::rc::Rc;

pub struct CpuBus<'a> {
    cart: &'a mut Cartridge,
    ram:  &'a mut Ram,
    ppu:  &'a mut Ppu,
    dma:  &'a mut Dma,
}

impl<'a> CpuBus<'a> {
    pub fn new(
        cart: &'a mut Cartridge,
        ram: &'a mut Ram,
        ppu: &'a mut Ppu,
        dma: &'a mut Dma,
    ) -> Self {
        Self {
            cart,
            ram,
            ppu,
            dma,
        }
    }

    pub fn read(&mut self, addr: Addr) -> Byte {
        match addr {
            Addr(0x0000..=0x1FFF) => self.ram.read(addr),
            Addr(0x2000..=0x3FFF) => self.ppu.read(self.cart, addr),
            Addr(0x4016) => unimplemented!(), // TODO: self.joy.read(),
            Addr(0x4017) => Byte(0),          // TODO: 2 player
            Addr(0x4000..=0x4017) => unimplemented!(), // TODO: self.apu.read(addr - 0x4000.into()),
            Addr(0x4018..=0x401F) => Byte(0), // Normally disabled. Enabled if CPU in test mode
            Addr(0x4020..=0xFFFF) => self.cart.read(addr),
            _ => panic!("[CPUBUS] Read from an illegal address ({:#06X})", addr),
        }
    }

    pub fn write(&mut self, addr: Addr, v: Byte) {
        match addr {
            Addr(0x0000..=0x1FFF) => self.ram.write(addr, v),
            Addr(0x2000..=0x3FFF) => self.ppu.write(self.cart, addr, v),
            Addr(0x4014) => unimplemented!(), // TODO: self.dma.write(v),
            Addr(0x4016) => unimplemented!(), // TODO: self.joy.write(v),
            Addr(0x4000..=0x4017) => unimplemented!(), // TODO: self.apu.write(addr - 0x4000.into(), v),
            Addr(0x4018..=0x401F) => { /*do nothing*/ } // Normally disabled. Enabled if CPU in test mode
            Addr(0x4020..=0xFFFF) => self.cart.write(addr, v),
            _ => panic!("[CPUBUS] Write to an illegal address ({:#06X})", addr),
        }
    }
}
