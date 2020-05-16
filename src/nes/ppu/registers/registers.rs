use super::{
    oam::Oam, ppuaddr::PpuAddr, ppuctrl::PpuCtrl, ppudata::PpuData, ppumask::PpuMask,
    ppuscroll::PpuScroll, ppustatus::PpuStatus,
};
use crate::ppu::context::Context;
use crate::prelude::*;

pub struct Registers {
    pub ppu_ctrl:   PpuCtrl,
    pub ppu_mask:   PpuMask,
    pub ppu_status: PpuStatus,
    pub oam:        Oam,
    pub ppu_scroll: PpuScroll,
    pub ppu_addr:   PpuAddr,
    pub ppu_data:   PpuData,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ppu_ctrl:   PpuCtrl::new(),
            ppu_mask:   PpuMask::new(),
            ppu_status: PpuStatus::new(),
            oam:        Oam::new(),
            ppu_scroll: PpuScroll::new(),
            ppu_addr:   PpuAddr::new(),
            ppu_data:   PpuData::new(),
        }
    }

    fn normalize_addr(addr: Addr) -> Addr {
        addr & 0x0007.into()
    }

    fn read_ppu_status(&mut self) -> Byte {
        let v = self.ppu_status;

        self.ppu_status.set_vertical_blank(false);
        self.ppu_status.set_sprite_zero_hit(false);
        self.ppu_scroll.set_axis_x();
        self.ppu_addr.set_part_hi();

        v.into()
    }

    fn read_ppu_data(&mut self, ctx: &mut Context) -> Byte {
        let addr = self.ppu_addr.addr();
        let res = self.ppu_data.read(ctx, addr);
        let offset = if self.ppu_ctrl.increment_mode() {
            Addr(32)
        } else {
            Addr(1)
        };
        self.ppu_addr.update(offset);

        res
    }

    fn write_ppu_data(&mut self, ctx: &mut Context, v: Byte) {
        let addr = self.ppu_addr.addr();
        self.ppu_data.write(ctx, addr, v);
        let offset = if self.ppu_ctrl.increment_mode() {
            Addr(32)
        } else {
            Addr(1)
        };
        self.ppu_addr.update(offset);
    }

    pub fn read(&mut self, ctx: &mut Context, addr: Addr) -> Byte {
        let addr = Registers::normalize_addr(addr);
        match addr {
            Addr(0x0000) => Byte(0),
            Addr(0x0001) => Byte(0),
            Addr(0x0002) => self.read_ppu_status(),
            Addr(0x0003) => Byte(0),
            Addr(0x0004) => self.oam.read_data(&mut ctx.oam_mem),
            Addr(0x0005) => Byte(0),
            Addr(0x0006) => Byte(0),
            Addr(0x0007) => self.read_ppu_data(ctx),
            _ => Byte(0),
        }
    }

    pub fn write(&mut self, ctx: &mut Context, addr: Addr, v: Byte) {
        let addr = Registers::normalize_addr(addr);
        match addr {
            Addr(0x0000) => self.ppu_ctrl = v.into(),
            Addr(0x0001) => self.ppu_mask = v.into(),
            Addr(0x0002) => {}
            Addr(0x0003) => self.oam.write_addr(v),
            Addr(0x0004) => self.oam.write_data(&mut ctx.oam_mem, v),
            Addr(0x0005) => self.ppu_scroll.write(v),
            Addr(0x0006) => self.ppu_addr.write(v),
            Addr(0x0007) => self.write_ppu_data(ctx, v),
            _ => {}
        };
    }
}
