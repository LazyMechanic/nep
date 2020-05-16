use crate::cartridge::Cartridge;
use crate::nes::ppu::Ppu;
use crate::prelude::*;
use crate::ram::internal_ram::InternalRam;
use crate::ram::Ram;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Bus {
    cart: Rc<RefCell<Cartridge>>,
    ram:  Rc<RefCell<InternalRam>>,
    ppu:  Rc<RefCell<Ppu>>,
}

impl Bus {
    pub fn new(
        cart: Rc<RefCell<Cartridge>>,
        ram: Rc<RefCell<InternalRam>>,
        ppu: Rc<RefCell<Ppu>>,
    ) -> Self {
        Self { cart, ram, ppu }
    }

    pub fn read(&mut self, addr: Addr) -> Byte {
        match addr {
            Addr(0x0000..=0x1FFF) => self.ram.borrow_mut().read(addr),
            Addr(0x2000..=0x3FFF) => self.ppu.borrow_mut().read(addr),
            Addr(0x4016) => unimplemented!(), // TODO: self.joy.read(),
            Addr(0x4017) => Byte(0),          // TODO: 2 player
            Addr(0x4000..=0x4017) => unimplemented!(), // TODO: self.apu.read(addr - 0x4000.into()),
            Addr(0x4018..=0x401F) => Byte(0), // Normally disabled. Enabled if CPU in test mode
            Addr(0x4020..=0xFFFF) => self.cart.borrow_mut().read(addr),
            // // Read ROM
            // //   0b1000'1001'1001'0011 {0x8993}
            // // - 0b1000'0000'0000'0000 {0x8000}
            // // = 0b0000'1001'1001'0011 {0x0993}
            // Addr(0x8000..=0xBFFF) => self.rom.read(addr - 0x8000.into()),
            // // Read ROM if small cartridge (<= 16kb)
            // //   0b1100'1001'1001'0011 {0xC993}
            // // - 0b1100'0000'0000'0000 {0xC000}
            // // = 0b0000'1001'1001'0011 {0x0993}
            // Addr(0xC000..=0xFFFF) if self.rom.size() <= 16_384 /* bytes */ => {
            //     self.rom.read(addr - 0xC000.into())
            // }
            // // Read ROM in other case
            // //   0b1100'1001'1001'0011 {0xC993}
            // // - 0b1000'0000'0000'0000 {0x8000}
            // // = 0b0100'1001'1001'0011 {0x4993}
            // Addr(0xC000..=0xFFFF) => self.rom.read(addr - 0x8000.into()),
            _ => panic!("[CPUBUS] Read from an illegal address ({:#06X})", addr),
        }
    }

    pub fn write(&mut self, addr: Addr, v: Byte) {
        match addr {
            Addr(0x0000..=0x1FFF) => self.ram.borrow_mut().write(addr, v),
            Addr(0x2000..=0x3FFF) => self.ppu.borrow_mut().write(addr, v),
            Addr(0x4014) => unimplemented!(), // TODO: self.dma.write(v),
            Addr(0x4016) => unimplemented!(), // TODO: self.joy.write(v),
            Addr(0x4000..=0x4017) => unimplemented!(), // TODO: self.apu.write(addr - 0x4000.into(), v),
            Addr(0x4018..=0x401F) => { /*do nothing*/ } // Normally disabled. Enabled if CPU in test mode
            Addr(0x4020..=0xFFFF) => self.cart.borrow_mut().write(addr, v),
            _ => panic!("[CPUBUS] Write to an illegal address ({:#06X})", addr),
        }
    }
}
