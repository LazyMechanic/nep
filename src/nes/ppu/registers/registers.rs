use super::{PpuCtrl, PpuMask, PpuStatus};
use crate::prelude::*;

struct Registers {
    pub ppu_ctrl:   PpuCtrl,
    pub ppu_mask:   PpuMask,
    pub ppu_status: PpuStatus,
    pub oam_addr:   Addr,
}
