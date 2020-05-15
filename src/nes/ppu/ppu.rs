use crate::prelude::*;

#[derive(Default)]
pub struct Ppu {}

impl Ppu {
    pub fn reset(&mut self) {}

    pub fn step(&mut self) {}

    pub fn read(&mut self, addr: Addr) -> Byte {
        unimplemented!()
    }

    pub fn write(&mut self, addr: Addr, v: Byte) {
        unimplemented!()
    }
}
