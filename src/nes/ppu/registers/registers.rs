use super::{ppuaddr::PpuAddr, ppuctrl::PpuCtrl, ppumask::PpuMask, ppustatus::PpuStatus};
use crate::prelude::*;

pub struct Registers {
    pub ppu_ctrl:   PpuCtrl,
    pub ppu_mask:   PpuMask,
    pub ppu_status: PpuStatus,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ppu_ctrl:   PpuCtrl::new(),
            ppu_mask:   PpuMask::new(),
            ppu_status: PpuStatus::new(),
        }
    }
}
