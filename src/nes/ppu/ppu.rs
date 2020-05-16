use super::context::Context;
use super::registers::Registers;
use crate::nes::cartridge::Cartridge;
use crate::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Ppu {
    regs: Registers,
    ctx:  Context,
}

impl Ppu {
    pub fn new(cart: Rc<RefCell<Cartridge>>) -> Self {
        Self {
            regs: Registers::new(),
            ctx:  Context::new(cart),
        }
    }

    pub fn reset(&mut self, cart: Rc<RefCell<Cartridge>>) {
        self.ctx.cart = cart;
    }

    pub fn step(&mut self) {}

    pub fn read(&mut self, addr: Addr) -> Byte {
        self.regs.read(&mut self.ctx, addr)
    }

    pub fn write(&mut self, addr: Addr, v: Byte) {
        self.regs.write(&mut self.ctx, addr, v)
    }

    pub fn read_chr(&mut self, addr: Addr) -> Byte {
        unimplemented!()
    }

    pub fn write_chr(&mut self, addr: Addr, v: Byte) {
        unimplemented!()
    }

    pub fn transfer_sprite(&mut self, addr: Addr, v: Byte) {
        let addr = addr + self.regs.oam.addr();
        self.ctx.palette_ram.write(addr & 0x0FFF.into(), v);
    }
}
