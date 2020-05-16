use crate::prelude::*;

pub trait Ram {
    fn read(&self, addr: Addr) -> Byte;
    fn write(&mut self, addr: Addr, v: Byte);
    fn size(&self) -> usize;
    fn dump(&self) -> &Vec<Byte>;
}
