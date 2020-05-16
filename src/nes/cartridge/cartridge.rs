use super::mappers::{Mapper, Mapper000};
use crate::prelude::*;

use std::cmp;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::str;

use std::io::prelude::*;
use std::io::SeekFrom;

#[derive(Debug, Clone, Copy)]
pub enum Mirror {
    Horizontal,
    Vertical,
    Hardware,
}

pub struct Cartridge {
    prg_mem: Vec<Byte>,
    chr_mem: Vec<Byte>,
    mirror:  Mirror,
    mapper:  Option<Box<dyn Mapper>>,
}

const PROGRAM_ROM_SIZE: usize = 16384; // 16 kb
const CHARACTER_ROM_SIZE: usize = 8192; // 8 kb
const PROGRAM_RAM_SIZE: usize = 8192; // 8 kb
const CHARACTER_RAM_SIZE: usize = 8192; // 8 kb
const HEADER_SIZE: usize = 16; // 2 byte

impl Cartridge {
    pub fn new() -> Self {
        Self {
            prg_mem: vec![Byte(0); 0],
            chr_mem: vec![Byte(0); 0],
            mirror:  Mirror::Hardware,
            mapper:  None,
        }
    }

    pub fn from_file<P>(file_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut s = Self::new();
        s.load(file_path)?;
        Ok(s)
    }

    pub fn load<P>(&mut self, file_path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        println!("[CARTGE] start read file: {}", file_path.as_ref().display());
        // iNES header format (16 bytes):
        // 0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
        // 4: Size of PRG ROM in 16 KB units
        // 5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
        // 6: Flags 6 - Mapper, mirroring, battery, trainer
        // 7: Flags 7 - Mapper, VS/Playchoice, NES 2.0
        // 8: Flags 8 - PRG-RAM size (rarely used extension)
        // 9: Flags 9 - TV system (rarely used extension)
        // 10: Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)
        // 11-15: Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)

        // Flags 6
        // 76543210
        // ||||||||
        // |||||||+- Mirroring: 0: horizontal (vertical arrangement) (CIRAM A10 = PPU A11)
        // |||||||              1: vertical (horizontal arrangement) (CIRAM A10 = PPU A10)
        // ||||||+-- 1: Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory
        // |||||+--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)
        // ||||+---- 1: Ignore mirroring control or above mirroring bit; instead provide four-screen VRAM
        // ++++----- Lower nybble of mapper number

        // Flags 7
        // 76543210
        // ||||||||
        // |||||||+- VS Unisystem
        // ||||||+-- PlayChoice-10 (8KB of Hint Screen data stored after CHR data)
        // ||||++--- If equal to 2, flags 8-15 are in NES 2.0 format
        // ++++----- Upper nybble of mapper number

        // Flags 8
        // 76543210
        // ||||||||
        // ++++++++- PRG RAM size

        // Flags 9
        // 76543210
        // ||||||||
        // |||||||+- TV system (0: NTSC; 1: PAL)
        // +++++++-- Reserved, set to zero

        // Flags 10
        // 76543210
        //   ||  ||
        //   ||  ++- TV system (0: NTSC; 2: PAL; 1/3: dual compatible)
        //   |+----- PRG RAM ($6000-$7FFF) (0: present; 1: not present)
        //   +------ 0: Board has no bus conflicts; 1: Board has bus conflicts

        let mut file = File::open(file_path).context(errors::OpenFile)?;

        let mut header_buf: [u8; HEADER_SIZE] = [0; HEADER_SIZE];
        let header_size = file.read(&mut header_buf).context(errors::ReadFile)?;

        if header_size < 16 {
            return errors::ReadCartridge {
                detail: format!(
                    "cannot read iNES header, file size less 16 byte, size = {}",
                    header_size
                ),
            }
            .fail();
        }

        let name = &header_buf[0..3];
        let ines = str::from_utf8(&name).unwrap();
        if ines != "NES" {
            return errors::ReadCartridge {
                detail: format!(
                    "cannot read iNES header, invalid name constant, [0..3] = {}",
                    ines
                ),
            }
            .fail();
        }

        let prg_rom_banks = header_buf[4] as usize;
        println!("[CARTGE] program rom banks: {}", prg_rom_banks);

        let chr_rom_banks = header_buf[5] as usize;
        println!("[CARTGE] character rom banks: {}", chr_rom_banks);

        let flags_6 = header_buf[6];
        println!("[CARTGE] flags 6: {0:#010b} ({0:#04x})", flags_6);
        multiline_println!(
            "                    ||||||||",
            "                    |||||||+- Mirroring: 0: horizontal (vertical arrangement) (CIRAM A10 = PPU A11)",
            "                    |||||||              1: vertical (horizontal arrangement) (CIRAM A10 = PPU A10)",
            "                    ||||||+-- 1: Cartridge contains battery-backed PRG RAM ($6000-7FFF) or other persistent memory",
            "                    |||||+--- 1: 512-byte trainer at $7000-$71FF (stored before PRG data)",
            "                    ||||+---- 1: Ignore mirroring control or above mirroring bit; instead provide four-screen VRAM",
            "                    ++++----- Lower nybble of mapper number"
        );
        println!();

        let flags_7 = header_buf[7];
        println!("[CARTGE] flags 7: {0:#010b} ({0:#04x})", flags_7);
        multiline_println!(
             "                    ||||||||",
             "                    |||||||+- VS Unisystem",
             "                    ||||||+-- PlayChoice-10 (8KB of Hint Screen data stored after CHR data)",
             "                    ||||++--- If equal to 2, flags 8-15 are in NES 2.0 format",
             "                    ++++----- Upper nybble of mapper number");
        println!();

        let flags_8 = header_buf[8] as usize;
        println!("[CARTGE] flags 8: {0:#010b} ({0:#04x})", flags_8);
        multiline_println!(
            "                    ||||||||",
            "                    ++++++++- PRG RAM size"
        );
        println!();

        let flags_9 = header_buf[9] as usize;
        println!("[CARTGE] flags 9: {0:#010b} ({0:#04x})", flags_9);
        multiline_println!(
            "                    ||||||||",
            "                    |||||||+- TV system (0: NTSC; 1: PAL)",
            "                    +++++++-- Reserved, set to zero"
        );
        println!();

        let flags_10 = header_buf[10] as usize;
        println!("[CARTGE] flags 10: {0:#010b} ({0:#04x})", flags_10);
        multiline_println!(
             "                      ||  ||",
             "                      ||  ++- TV system (0: NTSC; 2: PAL; 1/3: dual compatible)",
             "                      |+----- PRG RAM ($6000-$7FFF) (0: present; 1: not present)",
             "                      +------ 0: Board has no bus conflicts; 1: Board has bus conflicts");
        println!();

        // If a "trainer" exists we just need to read past
        // it before we get to the good stuff
        if flags_6 & 0b0000_0100 != 0 {
            file.seek(SeekFrom::Current(512));
        }

        // Determine mapper id
        //   0bAAAAxxxx (flags_7)
        // | 0bxxxxBBBB (flags_6)
        // = 0bAAAABBBB
        let mapper_id = (flags_7 & 0xF0) | ((flags_6 >> 4) & 0x0F);

        let file_type = flags_7 & 0b0000_1100;
        const NES_2_0: u8 = 0b0000_1000;

        // Determine program ram size
        let prg_ram_size = flags_8;

        // Mirroring
        let mirror = if flags_6 & 0x01 != 0x00 {
            Mirror::Vertical
        } else {
            Mirror::Horizontal
        };

        let (prg_mem, prg_banks) = {
            let banks = if file_type == NES_2_0 {
                // NES 2.0 FORMAT
                ((prg_ram_size & 0b0000_0000_0000_0111) << 8) | prg_rom_banks
            } else {
                // NOT NES 2.0 FORMAT
                cmp::max(1, prg_rom_banks)
            };

            // banks * 16kb
            let s = banks * PROGRAM_ROM_SIZE;
            let mut v: Vec<u8> = Vec::new();
            v.resize(s, 0);
            file.read(&mut v);

            (v.into_iter().map(Byte).collect(), banks)
        };

        let (chr_mem, chr_banks) = {
            let banks = if file_type == NES_2_0 {
                // NES 2.0 FORMAT
                ((prg_ram_size & 0b0000_0000_0011_1000) << 8) | prg_rom_banks
            } else {
                // NOT NES 2.0 FORMAT
                // If banks eq 0 than chr_mem is RAM, otherwise is ROM
                cmp::max(1, chr_rom_banks)
            };

            // banks * 8kb
            let s = banks * CHARACTER_ROM_SIZE;
            let mut v: Vec<u8> = Vec::new();
            v.resize(s, 0);
            file.read(&mut v);

            (v.into_iter().map(Byte).collect(), banks)
        };

        let mapper: Box<dyn Mapper> = match mapper_id {
            0 => Box::new(Mapper000::new(prg_banks, chr_banks)),
            _ => {
                return errors::ReadCartridge {
                    detail: format!("unknown mapper id = {}", mapper_id),
                }
                .fail();
            }
        };

        self.prg_mem = prg_mem;
        self.chr_mem = chr_mem;
        self.mirror = mirror;
        self.mapper = Some(mapper);

        Ok(())
    }

    pub fn read(&mut self, addr: Addr) -> Byte {
        let mut mapped_addr = ExtAddr(0xFFFF_FFFF);
        let mut value = Byte(0);

        match self.mapper {
            Some(ref mut m) => {
                if m.map_read(addr, &mut mapped_addr, &mut value) {
                    if mapped_addr == 0xFFFF_FFFF.into() {
                        // Mapper has actually set the data value, for example cartridge based RAM
                        // Do nothing
                    } else {
                        // Mapper has produced an offset into cartridge bank memory
                        value = self.prg_mem[mapped_addr.as_usize()];
                    }
                }
            }
            _ => {}
        };

        value
    }

    pub fn write(&mut self, addr: Addr, v: Byte) {
        let mut mapped_addr = ExtAddr(0xFFFF_FFFF);

        match self.mapper {
            Some(ref mut m) => {
                if m.map_write(addr, &mut mapped_addr, v) {
                    if mapped_addr == 0xFFFF_FFFF.into() {
                        // Mapper has actually set the data value, for example cartridge based RAM
                        // Do nothing
                    } else {
                        // Mapper has produced an offset into cartridge bank memory
                        self.prg_mem[mapped_addr.as_usize()] = v;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn read_chr(&mut self, addr: Addr) -> Byte {
        let mut mapped_addr = ExtAddr(0xFFFF_FFFF);
        let mut value = Byte(0);

        match self.mapper {
            Some(ref mut m) => {
                if m.map_read_chr(addr, &mut mapped_addr) {
                    // Mapper has produced an offset into cartridge bank memory
                    value = self.chr_mem[mapped_addr.as_usize()]
                }
            }
            _ => {}
        };

        value
    }

    pub fn write_chr(&mut self, addr: Addr, v: Byte) {
        let mut mapped_addr = ExtAddr(0xFFFF_FFFF);

        match self.mapper {
            Some(ref mut m) => {
                if m.map_write_chr(addr, &mut mapped_addr) {
                    // Mapper has produced an offset into cartridge bank memory
                    self.chr_mem[mapped_addr.as_usize()] = v;
                }
            }
            _ => {}
        }
    }

    pub fn mirror(&self) -> Mirror {
        let mapper_mirror = match self.mapper {
            Some(ref m) => m.mirror(),
            _ => Mirror::Hardware,
        };

        return match mapper_mirror {
            Mirror::Hardware => self.mirror,
            _ => mapper_mirror,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_from_file() -> Result<()> {
        const FILE_PATH: &str = "./roms/nestest.nes";
        let cartridge = Cartridge::from_file(FILE_PATH)?;
        Ok(())
    }
}
