use super::oam::{Oam, OamEntry};
use crate::nes::cartridge::Cartridge;
use crate::prelude::*;

pub struct Ppu {
    oam: Oam,
    nmi: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            oam: Oam::new(),
            nmi: false,
        }
    }

    pub fn reset(&mut self) {}

    pub fn oam_mut(&mut self) -> &mut Oam {
        &mut self.oam
    }

    pub fn has_nmi(&self) -> bool {
        self.nmi
    }

    pub fn clear_nmi(&mut self) {
        self.nmi = false;
    }

    pub fn step(&mut self, cart: &mut Cartridge) {
        //todo!();
    }

    pub fn read(&mut self, cart: &mut Cartridge, addr: Addr) -> Byte {
        //todo!();
        Byte(0)
    }

    pub fn write(&mut self, cart: &mut Cartridge, addr: Addr, v: Byte) {
        //todo!();
    }

    fn read_chr(&mut self, cart: &mut Cartridge, addr: Addr) -> Byte {
        //todo!();
        Byte(0)
    }

    fn write_chr(&mut self, cart: &mut Cartridge, addr: Addr, v: Byte) {
        //todo!();
    }
}
