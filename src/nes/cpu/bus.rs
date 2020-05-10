use crate::prelude::*;

pub trait CpuBus {
    fn read(&mut self, addr: Addr) -> Byte;
    fn write(&mut self, addr: Addr, v: Byte);
}

pub struct Bus {}

impl CpuBus for Bus {
    fn read(&mut self, addr: Addr) -> Byte {
        unimplemented!()
    }

    fn write(&mut self, addr: Addr, v: Byte) {
        unimplemented!()
    }
}
