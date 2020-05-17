use super::oam::{Oam, OamEntry};
use super::registers::{PpuCtrl, PpuMask, PpuStatus};
use crate::nes::cartridge::Cartridge;
use crate::prelude::*;

const TABLE_NAME_SIZE: usize = 1024;
const TABLE_NAME_COUNT: usize = 2;

const TABLE_PATTERN_SIZE: usize = 4096;
const TABLE_PATTERN_COUNT: usize = 2;

const TABLE_PALETTE_SIZE: usize = 32;

// Address range Size   Description
// --------------------------------------------
// $0000-$0FFF   $1000  Pattern table 0
// $1000-$1FFF   $1000  Pattern table 1
// --------------------------------------------
// $2000-$23FF   $0400  Nametable 0
// $2400-$27FF   $0400  Nametable 1
// $2800-$2BFF   $0400  Nametable 2
// $2C00-$2FFF   $0400  Nametable 3
// $3000-$3EFF   $0F00  Mirrors of $2000-$2EFF
// --------------------------------------------
// $3F00-$3F1F   $0020  Palette RAM indexes
// $3F20-$3FFF   $00E0  Mirrors of $3F00-$3F1F
// --------------------------------------------

pub struct Ppu {
    tbl_name:    Vec<Vec<Byte>>,
    tbl_pattern: Vec<Vec<Byte>>,
    tbl_palette: Vec<Byte>,

    status:  PpuStatus,
    mask:    PpuMask,
    control: PpuCtrl,

    oam: Oam,
    nmi: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            tbl_name:    vec![vec![Byte(0); TABLE_NAME_SIZE]; TABLE_NAME_COUNT],
            tbl_pattern: vec![vec![Byte(0); TABLE_PATTERN_SIZE]; TABLE_PATTERN_COUNT],
            tbl_palette: vec![Byte(0); TABLE_PALETTE_SIZE],
            status:      PpuStatus::new(),
            mask:        PpuMask::new(),
            control:     PpuCtrl::new(),
            oam:         Oam::new(),
            nmi:         false,
        }
    }

    pub fn reset(&mut self) {}

    pub fn oam_mut(&mut self) -> &mut Oam {
        &mut self.oam
    }

    pub fn has_nmi(&self) -> bool {
        self.nmi
    }

    pub fn clear_nmi(&mut self) {
        self.nmi = false;
    }

    pub fn step(&mut self, cart: &mut Cartridge) {
        //todo!();
    }

    pub fn read(&mut self, cart: &mut Cartridge, addr: Addr) -> Byte {
        //todo!();
        Byte(0)
    }

    pub fn write(&mut self, cart: &mut Cartridge, addr: Addr, v: Byte) {
        //todo!();
    }

    fn read_chr(&mut self, cart: &mut Cartridge, addr: Addr) -> Byte {
        //todo!();
        Byte(0)
    }

    fn write_chr(&mut self, cart: &mut Cartridge, addr: Addr, v: Byte) {
        //todo!();
    }
}
