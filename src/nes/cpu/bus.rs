use crate::types::*;

pub trait CpuBus {
    fn read(&mut self, addr: Addr) -> Byte;
    fn read_word(&mut self, addr: Addr) -> Word;
    fn write(&mut self, addr: Addr, v: Byte);
}

pub struct Bus {}

impl CpuBus for Bus {
    fn read(&mut self, addr: Addr) -> Byte { unimplemented!() }

    fn read_word(&mut self, addr: Addr) -> Word { unimplemented!() }

    fn write(&mut self, addr: Addr, v: Byte) { unimplemented!() }
}
