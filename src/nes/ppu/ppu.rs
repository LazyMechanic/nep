use super::oam::{Oam, OamEntry};
use crate::nes::cartridge::Cartridge;
use crate::prelude::*;

pub struct Ppu {
    oam: Oam,
}

impl Ppu {
    pub fn new() -> Self {
        Self { oam: Oam::new() }
    }

    pub fn reset(&mut self) {}

    pub fn step(&mut self, cart: &mut Cartridge) {
        todo!();
    }

    pub fn oam_mut(&mut self) -> &mut Oam {
        &mut self.oam
    }

    pub fn read(&mut self, cart: &mut Cartridge, addr: Addr) -> Byte {
        unimplemented!()
    }

    pub fn write(&mut self, cart: &mut Cartridge, addr: Addr, v: Byte) {
        unimplemented!()
    }

    fn read_chr(&mut self, cart: &mut Cartridge, addr: Addr) -> Byte {
        unimplemented!()
    }

    fn write_chr(&mut self, cart: &mut Cartridge, addr: Addr, v: Byte) {
        unimplemented!()
    }
}
