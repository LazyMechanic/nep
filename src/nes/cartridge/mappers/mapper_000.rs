use super::mapper::Mapper;

pub struct Mapper000 {
    prg_banks: usize,
    chr_banks: usize,
}

impl Mapper for Mapper000 {}

impl Mapper000 {
    pub fn new(prg_banks: usize, chr_banks: usize) -> Self {
        Self {
            prg_banks,
            chr_banks,
        }
    }
}
