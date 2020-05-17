use crate::cartridge::Mirror;
use crate::prelude::*;

pub trait Mapper {
    fn map_read(&mut self, addr: Addr, mapped_addr: &mut ExtAddr, v: &mut Byte) -> bool;
    fn map_write(&mut self, addr: Addr, mapped_addr: &mut ExtAddr, v: Byte) -> bool;
    fn map_read_chr(&mut self, addr: Addr, mapped_addr: &mut ExtAddr) -> bool;
    fn map_write_chr(&mut self, addr: Addr, mapped_addr: &mut ExtAddr) -> bool;
    fn mirror(&self) -> Mirror {
        Mirror::Hardware
    }
    fn has_irq(&self) -> bool {
        false
    }
    fn clear_irq(&mut self) {}
    fn scanline(&mut self) {}
}
