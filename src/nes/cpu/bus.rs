use crate::types::*;

pub trait CpuBus {
    fn read(&mut self, addr: Addr) -> Data;
    fn read_word(&mut self, addr: Addr) -> Word;
    fn write(&mut self, addr: Addr, data: Data);
}

pub struct Bus {}
