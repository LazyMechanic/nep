use super::oam::{Oam, OamEntry};
use super::registers::{AddrReg, PpuCtrl, PpuMask, PpuStatus};
use crate::nes::cartridge::{Cartridge, Mirror};
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
    tbl_name:    [[Byte; TABLE_NAME_SIZE]; TABLE_NAME_COUNT],
    tbl_pattern: [[Byte; TABLE_PATTERN_SIZE]; TABLE_PATTERN_COUNT],
    tbl_palette: [Byte; TABLE_PALETTE_SIZE],

    status:  PpuStatus,
    mask:    PpuMask,
    control: PpuCtrl,

    vram_addr: AddrReg, // Active "pointer" address into nametable to extract background tile info
    tram_addr: AddrReg, // Temporary store of information to be "transferred" into "pointer" at various times

    // Pixel offset horizontally
    fine_x: Addr,

    // Internal communications
    addr_latch:   u8,
    ppu_data_buf: Byte,

    // Pixel "dot" position information
    scanline:  u16,
    cycle:     u16,
    odd_frame: bool,

    // Background rendering
    bg_next_title_id:      u8,
    bg_next_title_attr:    u8,
    bg_next_title_lsb:     u8,
    bg_next_title_msb:     u8,
    bg_shifter_pattern_lo: u16,
    bg_shifter_pattern_hi: u16,
    bg_shifter_attr_lo:    u16,
    bg_shifter_attr_hi:    u16,

    // Foreground "Sprite" rendering
    // The OAM is an additional memory internal to the PPU. It is
    // not connected via the any bus. It stores the locations of
    // 64off 8x8 (or 8x16) tiles to be drawn on the next frame.
    oam: Oam,

    // A register to store the address when the CPU manually communicates
    // with OAM via PPU registers. This is not commonly used because it
    // is very slow, and instead a 256-Byte DMA transfer is used. See
    // the Bus header for a description of this.
    oam_addr: Addr,

    sprite_scan_line:          [OamEntry; 8],
    sprite_count:              u8,
    sprite_shifter_pattern_lo: [u8; 8],
    sprite_shifter_pattern_hi: [u8; 8],

    // Sprite Zero Collision Flags
    sprite_zero_hit_possible:   bool,
    sprite_zero_being_rendered: bool,

    nmi: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            tbl_name: [[Byte(0); TABLE_NAME_SIZE]; TABLE_NAME_COUNT],
            tbl_pattern: [[Byte(0); TABLE_PATTERN_SIZE]; TABLE_PATTERN_COUNT],
            tbl_palette: [Byte(0); TABLE_PALETTE_SIZE],
            status: PpuStatus::new(),
            mask: PpuMask::new(),
            control: PpuCtrl::new(),
            vram_addr: AddrReg::new(),
            tram_addr: AddrReg::new(),
            fine_x: Addr(0),
            addr_latch: 0,
            ppu_data_buf: Byte(0),
            scanline: 0,
            cycle: 0,
            odd_frame: false,
            bg_next_title_id: 0,
            bg_next_title_attr: 0,
            bg_next_title_lsb: 0,
            bg_next_title_msb: 0,
            bg_shifter_pattern_lo: 0,
            bg_shifter_pattern_hi: 0,
            bg_shifter_attr_lo: 0,
            bg_shifter_attr_hi: 0,
            oam: Oam::new(),
            oam_addr: Addr(0),
            sprite_scan_line: [OamEntry::new(); 8],
            sprite_count: 0,
            sprite_shifter_pattern_lo: [0u8; 8],
            sprite_shifter_pattern_hi: [0u8; 8],
            sprite_zero_hit_possible: false,
            sprite_zero_being_rendered: false,
            nmi: false,
        }
    }

    fn normalize_addr(addr: Addr) -> Addr {
        addr & 0x0007.into()
    }

    fn normalize_addr_chr(addr: Addr) -> Addr {
        addr & 0x3FFF.into()
    }

    fn normalize_addr_pattern(addr: Addr) -> (Addr, Addr) {
        let table_num = (addr & 0x1000.into()) >> 12;
        let cell = addr & 0x0FFF.into();
        (table_num, cell)
    }

    fn normalize_addr_name(addr: Addr) -> Addr {
        addr & 0x0FFF.into()
    }

    fn normalize_addr_palette(addr: Addr) -> Addr {
        addr & 0x001F.into()
    }

    pub fn reset(&mut self) {
        self.fine_x = Addr(0);
        self.addr_latch = 0;
        self.ppu_data_buf = Byte(0);
        self.scanline = 0;
        self.cycle = 0;
        self.odd_frame = false;
        self.bg_next_title_id = 0;
        self.bg_next_title_attr = 0;
        self.bg_next_title_lsb = 0;
        self.bg_next_title_msb = 0;
        self.bg_shifter_pattern_lo = 0;
        self.bg_shifter_pattern_hi = 0;
        self.bg_shifter_attr_lo = 0;
        self.bg_shifter_attr_hi = 0;
    }

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
        let addr = Self::normalize_addr(addr);
        match addr {
            Addr(0x0000) => self.control.into(),
            Addr(0x0001) => self.mask.into(),
            Addr(0x0002) => {
                let res = self.status.into();

                self.status.set_vertical_blank(false);
                self.addr_latch = 0;

                res
            }
            Addr(0x0003) => Byte(0),
            Addr(0x0004) => self.oam.read(self.oam_addr),
            Addr(0x0005) => Byte(0),
            Addr(0x0006) => Byte(0),
            Addr(0x0007) => {
                // Reads from the NameTable ram get delayed one cycle,
                // so output buffer which contains the data from the
                // previous read request
                let mut res = self.ppu_data_buf;
                // then update the buffer for next time
                self.ppu_data_buf = self.read_chr(cart, self.vram_addr.into());
                // However, if the address was in the palette range, the
                // data is not delayed, so it returns immediately
                if self.vram_addr >= 0x3F00.into() {
                    res = self.ppu_data_buf;
                }
                // All reads from PPU data automatically increment the nametable
                // address depending upon the mode set in the control register.
                // If set to vertical mode, the increment is 32, so it skips
                // one whole nametable row; in horizontal mode it just increments
                // by 1, moving to the next column
                self.vram_addr += if self.control.increment_mode() {
                    32.into()
                } else {
                    1.into()
                };

                res
            }
            _ => Byte(0),
        }
    }

    pub fn write(&mut self, cart: &mut Cartridge, addr: Addr, v: Byte) {
        let addr = Self::normalize_addr(addr);
        match addr {
            Addr(0x0000) => {
                self.control = v.into();
                self.tram_addr.set_nametable_x(self.control.nametable_x());
                self.tram_addr.set_nametable_y(self.control.nametable_y());
            }
            Addr(0x0001) => self.mask = v.into(),
            Addr(0x0002) => {}
            Addr(0x0003) => self.oam_addr = v.as_lo_addr(),
            Addr(0x0004) => self.oam.write(self.oam_addr, v),
            Addr(0x0005) => {
                if self.addr_latch == 0 {
                    // First write to scroll register contains X offset in pixel space
                    // which we split into coarse and fine x values
                    self.fine_x = (v & 0x07.into()).as_lo_addr();
                    self.tram_addr.set_coarse_x((v >> 3).as_lo_addr());
                    self.addr_latch = 1;
                } else {
                    // First write to scroll register contains Y offset in pixel space
                    // which we split into coarse and fine Y values
                    self.tram_addr.set_fine_y((v & 0x07.into()).as_lo_addr());
                    self.tram_addr.set_coarse_x((v >> 3).as_lo_addr());
                    self.addr_latch = 1;
                }
            }
            Addr(0x0006) => {
                if self.addr_latch == 0 {
                    // PPU address bus can be accessed by CPU via the ADDR and DATA
                    // registers. The fisrt write to this register latches the high byte
                    // of the address, the second is the low byte. Note the writes
                    // are stored in the tram register...
                    self.tram_addr = ((v & 0x3F.into()).as_hi_addr()
                        | Addr::from(self.tram_addr).lo_addr())
                    .into();
                    self.addr_latch = 1;
                } else {
                    // ...when a whole address has been written, the internal vram address
                    // buffer is updated. Writing to the PPU is unwise during rendering
                    // as the PPU will maintam the vram address automatically whilst
                    // rendering the scanline position.
                    self.tram_addr = (Addr::from(self.tram_addr).hi_addr() | v.as_lo_addr()).into();
                    self.vram_addr = self.tram_addr;
                    self.addr_latch = 1;
                }
            }
            Addr(0x0007) => {
                self.write_chr(cart, self.vram_addr.into(), v);
                // All writes from PPU data automatically increment the nametable
                // address depending upon the mode set in the control register.
                // If set to vertical mode, the increment is 32, so it skips
                // one whole nametable row; in horizontal mode it just increments
                // by 1, moving to the next column
                self.vram_addr += if self.control.increment_mode() {
                    32.into()
                } else {
                    1.into()
                };
            }
            _ => {}
        };
    }

    fn read_chr(&mut self, cart: &mut Cartridge, addr: Addr) -> Byte {
        let addr = Self::normalize_addr_chr(addr);
        match addr {
            Addr(0x0000..=0x1FFF) => {
                let (v, mapped) = cart.read_chr(addr);
                if mapped {
                    v
                } else {
                    // If the cartridge cant map the address, have
                    // a physical location ready here
                    let (table_num, cell) = Self::normalize_addr_pattern(addr);
                    self.tbl_pattern[table_num.as_usize()][cell.as_usize()]
                }
            }
            Addr(0x2000..=0x3EFF) => {
                let addr = Self::normalize_addr_name(addr);
                let tbl_addr = addr & 0x03FF.into();
                match cart.mirror() {
                    Mirror::Horizontal => match addr {
                        Addr(0x0000..=0x03FF) => self.tbl_name[0][tbl_addr.as_usize()],
                        Addr(0x0400..=0x07FF) => self.tbl_name[1][tbl_addr.as_usize()],
                        Addr(0x0800..=0x0BFF) => self.tbl_name[0][tbl_addr.as_usize()],
                        Addr(0x0C00..=0x0FFF) => self.tbl_name[1][tbl_addr.as_usize()],
                        _ => Byte(0),
                    },
                    Mirror::Vertical => match addr {
                        Addr(0x0000..=0x03FF) => self.tbl_name[0][tbl_addr.as_usize()],
                        Addr(0x0400..=0x07FF) => self.tbl_name[0][tbl_addr.as_usize()],
                        Addr(0x0800..=0x0BFF) => self.tbl_name[1][tbl_addr.as_usize()],
                        Addr(0x0C00..=0x0FFF) => self.tbl_name[1][tbl_addr.as_usize()],
                        _ => Byte(0),
                    },
                    _ => Byte(0),
                }
            }
            Addr(0x3F00..=0x3FFF) => {
                let mut addr = Self::normalize_addr_palette(addr);
                addr = match addr {
                    Addr(0x0010) => Addr(0x0000),
                    Addr(0x0014) => Addr(0x0004),
                    Addr(0x0018) => Addr(0x0008),
                    Addr(0x001C) => Addr(0x000C),
                    _ => addr,
                };

                let res = self.tbl_palette[addr.as_usize()]
                    & if self.mask.grayscale() {
                        0x30.into()
                    } else {
                        0x3F.into()
                    };
                res
            }
            _ => Byte(0),
        }
    }

    fn write_chr(&mut self, cart: &mut Cartridge, addr: Addr, v: Byte) {
        let addr = Self::normalize_addr_chr(addr);
        match addr {
            Addr(0x0000..=0x1FFF) => {
                let mapped = cart.write_chr(addr, v);
                if mapped {
                    // Do nothing
                } else {
                    let (table_num, cell) = Self::normalize_addr_pattern(addr);
                    self.tbl_pattern[table_num.as_usize()][cell.as_usize()] = v;
                }
            }
            Addr(0x2000..=0x3EFF) => {
                let addr = Self::normalize_addr_name(addr);
                let tbl_addr = addr & 0x03FF.into();
                match cart.mirror() {
                    Mirror::Horizontal => match addr {
                        Addr(0x0000..=0x03FF) => self.tbl_name[0][tbl_addr.as_usize()] = v,
                        Addr(0x0400..=0x07FF) => self.tbl_name[1][tbl_addr.as_usize()] = v,
                        Addr(0x0800..=0x0BFF) => self.tbl_name[0][tbl_addr.as_usize()] = v,
                        Addr(0x0C00..=0x0FFF) => self.tbl_name[1][tbl_addr.as_usize()] = v,
                        _ => {}
                    },
                    Mirror::Vertical => match addr {
                        Addr(0x0000..=0x03FF) => self.tbl_name[0][tbl_addr.as_usize()] = v,
                        Addr(0x0400..=0x07FF) => self.tbl_name[0][tbl_addr.as_usize()] = v,
                        Addr(0x0800..=0x0BFF) => self.tbl_name[1][tbl_addr.as_usize()] = v,
                        Addr(0x0C00..=0x0FFF) => self.tbl_name[1][tbl_addr.as_usize()] = v,
                        _ => {}
                    },
                    _ => {}
                }
            }
            Addr(0x3F00..=0x3FFF) => {
                let mut addr = Self::normalize_addr_palette(addr);
                addr = match addr {
                    Addr(0x0010) => Addr(0x0000),
                    Addr(0x0014) => Addr(0x0004),
                    Addr(0x0018) => Addr(0x0008),
                    Addr(0x001C) => Addr(0x000C),
                    _ => addr,
                };

                self.tbl_palette[addr.as_usize()] = v;
            }
            _ => {}
        }
    }
}
