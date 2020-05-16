use crate::ppu::context::Context;
use crate::prelude::*;

#[derive(Debug)]
pub struct PpuData {
    buf: Byte,
}

impl PpuData {
    pub fn new() -> Self {
        Self { buf: Byte(0) }
    }

    pub fn read(&mut self, ctx: &mut Context, addr: Addr) -> Byte {
        let buf = self.buf;

        match addr {
            Addr(0x0000..=0x1FFF) => self.buf = ctx.cart.borrow_mut().read_chr(addr),
            Addr(0x2000..=0x3EFF) => {
                self.buf = ctx.name_table.read(addr, ctx.cart.borrow().mirror())
            }
            Addr(0x3F00..=0x3FFF) => {
                self.buf = ctx.name_table.read(addr, ctx.cart.borrow().mirror());
                return ctx.palette_ram.read(addr);
            }
            _ => {}
        }

        buf
    }

    pub fn write(&mut self, ctx: &mut Context, addr: Addr, v: Byte) {
        match addr {
            Addr(0x0000..=0x1FFF) => ctx.cart.borrow_mut().write_chr(addr, v),
            Addr(0x2000..=0x3EFF) => ctx.name_table.write(addr, v, ctx.cart.borrow().mirror()),
            Addr(0x3F00..=0x3FFF) => ctx.palette_ram.write(addr, v),
            _ => {}
        };
    }
}
