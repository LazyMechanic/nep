use super::name_table::NameTable;
use super::palette_ram::PaletteRam;
use super::pattern_table::PatternTable;
use crate::cartridge::Cartridge;
use crate::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

const OAM_SIZE: usize = 256;

pub struct Context {
    pub cart:          Rc<RefCell<Cartridge>>,
    pub name_table:    NameTable,
    pub pattern_table: PatternTable,
    pub palette_ram:   PaletteRam,
    pub oam_mem:       Vec<Byte>,
}

impl Context {
    pub fn new(cart: Rc<RefCell<Cartridge>>) -> Self {
        Self {
            cart,
            oam_mem: vec![Byte(0); OAM_SIZE],
            name_table: NameTable::new(),
            pattern_table: PatternTable::new(),
            palette_ram: PaletteRam::new(),
        }
    }
}
