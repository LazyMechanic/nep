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
    scanline:  i16,
    cycle:     i16,
    odd_frame: bool,

    // Background rendering
    bg_next_tile_id:       Byte,
    bg_next_tile_attr:     Byte,
    bg_next_tile_lsb:      Byte,
    bg_next_tile_msb:      Byte,
    bg_shifter_pattern_lo: Word,
    bg_shifter_pattern_hi: Word,
    bg_shifter_attr_lo:    Word,
    bg_shifter_attr_hi:    Word,

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
    sprite_count:              usize,
    sprite_shifter_pattern_lo: [Byte; 8],
    sprite_shifter_pattern_hi: [Byte; 8],

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
            bg_next_tile_id: Byte(0),
            bg_next_tile_attr: Byte(0),
            bg_next_tile_lsb: Byte(0),
            bg_next_tile_msb: Byte(0),
            bg_shifter_pattern_lo: Word(0),
            bg_shifter_pattern_hi: Word(0),
            bg_shifter_attr_lo: Word(0),
            bg_shifter_attr_hi: Word(0),
            oam: Oam::new(),
            oam_addr: Addr(0),
            sprite_scan_line: [OamEntry::new(); 8],
            sprite_count: 0,
            sprite_shifter_pattern_lo: [Byte(0); 8],
            sprite_shifter_pattern_hi: [Byte(0); 8],
            sprite_zero_hit_possible: false,
            sprite_zero_being_rendered: false,
            nmi: false,
        }
    }

    fn normalize_addr(addr: Addr) -> Addr {
        addr & Addr(0x0007)
    }

    fn normalize_addr_chr(addr: Addr) -> Addr {
        addr & Addr(0x3FFF)
    }

    fn normalize_addr_pattern(addr: Addr) -> (Addr, Addr) {
        let table_num = (addr & Addr(0x1000)) >> 12;
        let cell = addr & Addr(0x0FFF);
        (table_num, cell)
    }

    fn normalize_addr_name(addr: Addr) -> Addr {
        addr & Addr(0x0FFF)
    }

    fn normalize_addr_palette(addr: Addr) -> Addr {
        addr & Addr(0x001F)
    }

    pub fn reset(&mut self) {
        self.fine_x = Addr(0);
        self.addr_latch = 0;
        self.ppu_data_buf = Byte(0);
        self.scanline = 0;
        self.cycle = 0;
        self.odd_frame = false;
        self.bg_next_tile_id = Byte(0);
        self.bg_next_tile_attr = Byte(0);
        self.bg_next_tile_lsb = Byte(0);
        self.bg_next_tile_msb = Byte(0);
        self.bg_shifter_pattern_lo = Word(0);
        self.bg_shifter_pattern_hi = Word(0);
        self.bg_shifter_attr_lo = Word(0);
        self.bg_shifter_attr_hi = Word(0);
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
                let tbl_addr = addr & Addr(0x03FF);
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
                let tbl_addr = addr & Addr(0x03FF);
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

    // ==============================================================================
    // Increment the background tile "pointer" one tile/column horizontally
    fn increment_scroll_x(&mut self) {
        // Note: pixel perfect scrolling horizontally is handled by the
        // data shifters. Here we are operating in the spatial domain of
        // tiles, 8x8 pixel blocks.

        // Ony if rendering is enabled
        if self.mask.render_background() || self.mask.render_sprites() {
            // A single name table is 32x30 tiles. As we increment horizontally
            // we may cross into a neighbouring nametable, or wrap around to
            // a neighbouring nametable
            if self.vram_addr.coarse_x() == Addr(31) {
                // Leaving nametable so wrap address round
                self.vram_addr.set_coarse_x(Addr(0));
                // Flip target nametable bit
                self.vram_addr
                    .set_nametable_x(!self.vram_addr.nametable_x());
            } else {
                // Staying in current nametable, so just increment
                self.vram_addr.inc_coarse_x();
            }
        }
    }

    // ==============================================================================
    // Increment the background tile "pointer" one scanline vertically
    fn increment_scroll_y(&mut self) {
        // Incrementing vertically is more complicated. The visible nametable
        // is 32x30 tiles, but in memory there is enough room for 32x32 tiles.
        // The bottom two rows of tiles are in fact not tiles at all, they
        // contain the "attribute" information for the entire table. This is
        // information that describes which palettes are used for different
        // regions of the nametable.

        // In addition, the NES doesnt scroll vertically in chunks of 8 pixels
        // i.e. the height of a tile, it can perform fine scrolling by using
        // the fine_y component of the register. This means an increment in Y
        // first adjusts the fine offset, but may need to adjust the whole
        // row offset, since fine_y is a value 0 to 7, and a row is 8 pixels high

        // Ony if rendering is enabled
        if self.mask.render_background() || self.mask.render_sprites() {
            // If possible, just increment the fine y offset
            if self.vram_addr.fine_y() < Addr(7) {
                self.vram_addr.inc_fine_y();
            } else {
                // If we have gone beyond the height of a row, we need to
                // increment the row, potentially wrapping into neighbouring
                // vertical nametables. Dont forget however, the bottom two rows
                // do not contain tile information. The coarse y offset is used
                // to identify which row of the nametable we want, and the fine
                // y offset is the specific "scanline"

                // Reset fine y offset
                self.vram_addr.set_fine_y(Addr(0));

                // Check if we need to swap vertical nametable targets
                if self.vram_addr.coarse_y() == Addr(29) {
                    // We do, so reset coarse y offset
                    self.vram_addr.set_coarse_y(Addr(0));
                    // And flip the target nametable bit
                    self.vram_addr
                        .set_nametable_y(!self.vram_addr.nametable_y());
                } else if self.vram_addr.coarse_y() == Addr(31) {
                    // In case the pointer is in the attribute memory, we
                    // just wrap around the current nametable
                    self.vram_addr.set_coarse_y(Addr(0));
                } else {
                    // None of the above boundary/wrapping conditions apply
                    // so just increment the coarse y offset
                    self.vram_addr.inc_coarse_y();
                }
            }
        }
    }

    // ==============================================================================
    // Transfer the temporarily stored horizontal nametable access information
    // into the "pointer". Note that fine x scrolling is not part of the "pointer"
    // addressing mechanism
    fn transfer_address_x(&mut self) {
        // Ony if rendering is enabled
        if self.mask.render_background() || self.mask.render_sprites() {
            self.vram_addr.set_nametable_x(self.tram_addr.nametable_x());
            self.vram_addr.set_coarse_x(self.tram_addr.coarse_x());
        }
    }

    // ==============================================================================
    // Transfer the temporarily stored vertical nametable access information
    // into the "pointer". Note that fine y scrolling is part of the "pointer"
    // addressing mechanism
    fn transfer_address_y(&mut self) {
        // Ony if rendering is enabled
        if self.mask.render_background() || self.mask.render_sprites() {
            self.vram_addr.set_fine_y(self.tram_addr.fine_y());
            self.vram_addr.set_nametable_y(self.tram_addr.nametable_y());
            self.vram_addr.set_coarse_y(self.tram_addr.coarse_y());
        }
    }

    // ==============================================================================
    // Prime the "in-effect" background tile shifters ready for outputting next
    // 8 pixels in scanline.
    fn load_background_shifters(&mut self) {
        // Each PPU update we calculate one pixel. These shifters shift 1 bit along
        // feeding the pixel compositor with the binary information it needs. Its
        // 16 bits wide, because the top 8 bits are the current 8 pixels being drawn
        // and the bottom 8 bits are the next 8 pixels to be drawn. Naturally this means
        // the required bit is always the MSB of the shifter. However, "fine x" scrolling
        // plays a part in this too, whcih is seen later, so in fact we can choose
        // any one of the top 8 bits.
        self.bg_shifter_pattern_lo =
            (self.bg_shifter_pattern_lo & Word(0xFF00)) | (self.bg_next_tile_lsb.as_lo_word());
        self.bg_shifter_pattern_hi =
            (self.bg_shifter_pattern_hi & Word(0xFF00)) | (self.bg_next_tile_msb.as_lo_word());

        // Attribute bits do not change per pixel, rather they change every 8 pixels
        // but are synchronised with the pattern shifters for convenience, so here
        // we take the bottom 2 bits of the attribute word which represent which
        // palette is being used for the current 8 pixels and the next 8 pixels, and
        // "inflate" them to 8 bit words.
        self.bg_shifter_attr_lo = (self.bg_shifter_attr_lo & Word(0xFF00))
            | (if self.bg_next_tile_attr.inspect_bit(0) {
                Word(0x00FF)
            } else {
                Word(0x0000)
            });
        self.bg_shifter_attr_hi = (self.bg_shifter_attr_hi & Word(0xFF00))
            | (if self.bg_next_tile_attr.inspect_bit(0) {
                Word(0x00FF)
            } else {
                Word(0x0000)
            });
    }

    // ==============================================================================
    // Every cycle the shifters storing pattern and attribute information shift
    // their contents by 1 bit. This is because every cycle, the output progresses
    // by 1 pixel. This means relatively, the state of the shifter is in sync
    // with the pixels being drawn for that 8 pixel section of the scanline.
    fn update_shifters(&mut self) {
        if self.mask.render_background() {
            // Shifting background tile pattern row
            self.bg_shifter_pattern_lo <<= 1;
            self.bg_shifter_pattern_hi <<= 1;

            // Shifting palette attributes by 1
            self.bg_shifter_attr_lo <<= 1;
            self.bg_shifter_attr_hi <<= 1;
        }

        if self.mask.render_sprites() && self.cycle >= 1 && self.cycle < 258 {
            for i in 0..self.sprite_count {
                if self.sprite_scan_line[i].x > 0.into() {
                    self.sprite_scan_line[i].x -= 1.into();
                } else {
                    self.sprite_shifter_pattern_lo[i] <<= 1;
                    self.sprite_shifter_pattern_hi[i] <<= 1;
                }
            }
        }
    }

    pub fn step(&mut self, cart: &mut Cartridge) {
        // As we progress through scanlines and cycles, the PPU is effectively
        // a state machine going through the motions of fetching background
        // information and sprite information, compositing them into a pixel
        // to be output.

        // The lambda functions (functions inside functions) contain the various
        // actions to be performed depending upon the output of the state machine
        // for a given scanline/cycle combination

        // All but 1 of the secanlines is visible to the user. The pre-render scanline
        // at -1, is used to configure the "shifters" for the first visible scanline, 0.
        if self.scanline >= -1 && self.scanline < 240 {
            // Background Rendering ======================================================

            if self.scanline == 0
                && self.cycle == 0
                && self.odd_frame
                && (self.mask.render_background() || self.mask.render_sprites())
            {
                // "Odd Frame" cycle skip
                self.cycle = 1;
            }

            if self.scanline == -1 && self.cycle == 1 {
                // Effectively start of new frame, so clear vertical blank flag
                self.status.disable_vertical_blank();

                // Clear sprite overflow flag
                self.status.disable_sprite_overflow();

                // Clear the sprite zero hit flag
                self.status.disable_sprite_zero_hit();

                // Clear Shifters
                for i in 0..8 {
                    self.sprite_shifter_pattern_lo[i] = Byte(0);
                    self.sprite_shifter_pattern_hi[i] = Byte(0);
                }
            }

            if (self.cycle >= 2 && self.cycle < 258) || (self.cycle >= 321 && self.cycle < 338) {
                self.update_shifters();

                // In these cycles we are collecting and working with visible data
                // The "shifters" have been preloaded by the end of the previous
                // scanline with the data for the start of this scanline. Once we
                // leave the visible region, we go dormant until the shifters are
                // preloaded for the next scanline.

                // Fortunately, for background rendering, we go through a fairly
                // repeatable sequence of events, every 2 clock cycles.
                match (self.cycle - 1) % 8 {
                    0 => {
                        // Load the current background tile pattern and attributes into the "shifter"
                        self.load_background_shifters();

                        // Fetch the next background tile ID
                        // "(vram_addr.reg & 0x0FFF)" : Mask to 12 bits that are relevant
                        // "| 0x2000"                 : Offset into nametable space on PPU address bus
                        self.bg_next_tile_id = self.read_chr(
                            cart,
                            Addr(0x2000) | (Addr::from(self.vram_addr) & Addr(0x0FFF)),
                        );

                        // Explanation:
                        // The bottom 12 bits of the loopy register provide an index into
                        // the 4 nametables, regardless of nametable mirroring configuration.
                        // nametable_y(1) nametable_x(1) coarse_y(5) coarse_x(5)
                        //
                        // Consider a single nametable is a 32x32 array, and we have four of them
                        //   0                1
                        // 0 +----------------+----------------+
                        //   |                |                |
                        //   |                |                |
                        //   |    (32x32)     |    (32x32)     |
                        //   |                |                |
                        //   |                |                |
                        // 1 +----------------+----------------+
                        //   |                |                |
                        //   |                |                |
                        //   |    (32x32)     |    (32x32)     |
                        //   |                |                |
                        //   |                |                |
                        //   +----------------+----------------+
                        //
                        // This means there are 4096 potential locations in this array, which
                        // just so happens to be 2^12!
                    }
                    2 => {
                        // Fetch the next background tile attribute. OK, so this one is a bit
                        // more involved :P

                        // Recall that each nametable has two rows of cells that are not tile
                        // information, instead they represent the attribute information that
                        // indicates which palettes are applied to which area on the screen.
                        // Importantly (and frustratingly) there is not a 1 to 1 correspondance
                        // between background tile and palette. Two rows of tile data holds
                        // 64 attributes. Therfore we can assume that the attributes affect
                        // 8x8 zones on the screen for that nametable. Given a working resolution
                        // of 256x240, we can further assume that each zone is 32x32 pixels
                        // in screen space, or 4x4 tiles. Four system palettes are allocated
                        // to background rendering, so a palette can be specified using just
                        // 2 bits. The attribute byte therefore can specify 4 distinct palettes.
                        // Therefore we can even further assume that a single palette is
                        // applied to a 2x2 tile combination of the 4x4 tile zone. The very fact
                        // that background tiles "share" a palette locally is the reason why
                        // in some games you see distortion in the colours at screen edges.

                        // As before when choosing the tile ID, we can use the bottom 12 bits of
                        // the loopy register, but we need to make the implementation "coarser"
                        // because instead of a specific tile, we want the attribute byte for a
                        // group of 4x4 tiles, or in other words, we divide our 32x32 address
                        // by 4 to give us an equivalent 8x8 address, and we offset this address
                        // into the attribute section of the target nametable.

                        // Reconstruct the 12 bit loopy address into an offset into the
                        // attribute memory

                        // "(vram_addr.coarse_x >> 2)"        : integer divide coarse x by 4,
                        //                                      from 5 bits to 3 bits
                        // "((vram_addr.coarse_y >> 2) << 3)" : integer divide coarse y by 4,
                        //                                      from 5 bits to 3 bits,
                        //                                      shift to make room for coarse x

                        // Result so far: YX00 00yy yxxx

                        // All attribute memory begins at 0x03C0 within a nametable, so OR with
                        // result to select target nametable, and attribute byte offset. Finally
                        // OR with 0x2000 to offset into nametable address space on PPU bus.
                        self.bg_next_tile_attr = self.read_chr(
                            cart,
                            Addr(0x23C0)
                                | (self.vram_addr.nametable_y().as_addr() << 11)
                                | (self.vram_addr.nametable_x().as_addr() << 10)
                                | ((self.vram_addr.coarse_y() >> 2) << 3)
                                | (self.vram_addr.coarse_x() >> 2),
                        );

                        // Right we've read the correct attribute byte for a specified address,
                        // but the byte itself is broken down further into the 2x2 tile groups
                        // in the 4x4 attribute zone.

                        // The attribute byte is assembled thus: BR(76) BL(54) TR(32) TL(10)
                        //
                        // +----+----+			    +----+----+
                        // | TL | TR |			    | ID | ID |
                        // +----+----+ where TL =   +----+----+
                        // | BL | BR |			    | ID | ID |
                        // +----+----+			    +----+----+
                        //
                        // Since we know we can access a tile directly from the 12 bit address, we
                        // can analyse the bottom bits of the coarse coordinates to provide us with
                        // the correct offset into the 8-bit word, to yield the 2 bits we are
                        // actually interested in which specifies the palette for the 2x2 group of
                        // tiles. We know if "coarse y % 4" < 2 we are in the top half else bottom half.
                        // Likewise if "coarse x % 4" < 2 we are in the left half else right half.
                        // Ultimately we want the bottom two bits of our attribute word to be the
                        // palette selected. So shift as required...
                        if self.vram_addr.coarse_y() & Addr(0x0002) != Addr(0x0000) {
                            self.bg_next_tile_attr >>= 4;
                        }
                        if self.vram_addr.coarse_x() & Addr(0x0002) != Addr(0x0000) {
                            self.bg_next_tile_attr >>= 2;
                        }
                        self.bg_next_tile_attr &= Byte(0x03);
                    }

                    // Compared to the last two, the next two are the easy ones... :P
                    4 => {
                        // Fetch the next background tile LSB bit plane from the pattern memory
                        // The Tile ID has been read from the nametable. We will use this id to
                        // index into the pattern memory to find the correct sprite (assuming
                        // the sprites lie on 8x8 pixel boundaries in that memory, which they do
                        // even though 8x16 sprites exist, as background tiles are always 8x8).
                        //
                        // Since the sprites are effectively 1 bit deep, but 8 pixels wide, we
                        // can represent a whole sprite row as a single byte, so offsetting
                        // into the pattern memory is easy. In total there is 8KB so we need a
                        // 13 bit address.

                        // "(control.pattern_background << 12)"  : the pattern memory selector
                        //                                         from control register, either 0K
                        //                                         or 4K offset
                        // "((uint16_t)bg_next_tile_id << 4)"    : the tile id multiplied by 16, as
                        //                                         2 lots of 8 rows of 8 bit pixels
                        // "(vram_addr.fine_y)"                  : Offset into which row based on
                        //                                         vertical scroll offset
                        // "+ 0"                                 : Mental clarity for plane offset
                        // Note: No PPU address bus offset required as it starts at 0x0000
                        self.bg_next_tile_lsb = self.read_chr(
                            cart,
                            (self.control.pattern_background().as_addr() << 12)
                                + (self.bg_next_tile_id.as_lo_addr() << 4)
                                + self.vram_addr.fine_y()
                                + Addr(0),
                        );
                    }
                    6 => {
                        // Fetch the next background tile MSB bit plane from the pattern memory
                        // This is the same as above, but has a +8 offset to select the next bit plane
                        self.bg_next_tile_msb = self.read_chr(
                            cart,
                            (self.control.pattern_background().as_addr() << 12)
                                + (self.bg_next_tile_id.as_lo_addr() << 4)
                                + (self.vram_addr.fine_y())
                                + Addr(8),
                        );
                    }
                    7 => {
                        // Increment the background tile "pointer" to the next tile horizontally
                        // in the nametable memory. Note this may cross nametable boundaries which
                        // is a little complex, but essential to implement scrolling
                        self.increment_scroll_x();
                    }
                    _ => {}
                }
            }

            // End of a visible scanline, so increment downwards...
            if self.cycle == 256 {
                self.increment_scroll_y();
            }

            //...and reset the x position
            if self.cycle == 257 {
                self.load_background_shifters();
                self.transfer_address_x();
            }

            // Superfluous reads of tile id at end of scanline
            if self.cycle == 338 || self.cycle == 340 {
                self.bg_next_tile_id = self.read_chr(
                    cart,
                    Addr(0x2000) | (Addr::from(self.vram_addr) & Addr(0x0FFF)).into(),
                );
            }

            if self.scanline == -1 && self.cycle >= 280 && self.cycle < 305 {
                // End of vertical blank period so reset the Y address ready for rendering
                self.transfer_address_y();
            }

            // Foreground Rendering ========================================================
            // I'm gonna cheat a bit here, which may reduce compatibility, but greatly
            // simplifies delivering an intuitive understanding of what exactly is going
            // on. The PPU loads sprite information successively during the region that
            // background tiles are not being drawn. Instead, I'm going to perform
            // all sprite evaluation in one hit. THE NES DOES NOT DO IT LIKE THIS! This makes
            // it easier to see the process of sprite evaluation.
            if self.cycle == 257 && self.scanline >= 0 {
                // We've reached the end of a visible scanline. It is now time to determine
                // which sprites are visible on the next scanline, and preload this info
                // into buffers that we can work with while the scanline scans the row.

                // Firstly, clear out the sprite memory. This memory is used to store the
                // sprites to be rendered. It is not the OAM.
                for i in self.sprite_scan_line.iter_mut() {
                    i.x = 0x00.into();
                    i.y = 0x00.into();
                    i.id = 0x00.into();
                    i.attr = 0x00.into();
                }

                // The NES supports a maximum number of sprites per scanline. Nominally
                // this is 8 or fewer sprites. This is why in some games you see sprites
                // flicker or disappear when the scene gets busy.
                self.sprite_count = 0;

                // Secondly, clear out any residual information in sprite pattern shifters
                for i in 0..8 {
                    self.sprite_shifter_pattern_lo[i] = Byte(0);
                    self.sprite_shifter_pattern_hi[i] = Byte(0);
                }

                // Thirdly, Evaluate which sprites are visible in the next scanline. We need
                // to iterate through the OAM until we have found 8 sprites that have Y-positions
                // and heights that are within vertical range of the next scanline. Once we have
                // found 8 or exhausted the OAM we stop. Now, notice I count to 9 sprites. This
                // is so I can set the sprite overflow flag in the event of there being > 8 sprites.
                let mut oam_entry = Addr(0);

                // New set of sprites. Sprite zero may not exist in the new set, so clear this
                // flag.
                self.sprite_zero_hit_possible = false;

                while oam_entry < Addr(64) && self.sprite_count < 9 {
                    // Note the conversion to signed numbers here
                    let diff = self.scanline - i16::from(self.oam.read_entry(oam_entry).y);

                    // If the difference is positive then the scanline is at least at the
                    // same height as the sprite, so check if it resides in the sprite vertically
                    // depending on the current "sprite height mode"
                    // FLAGGED

                    if diff >= 0
                        && diff < (if self.control.sprite_size() { 16 } else { 8 })
                        && self.sprite_count < 8
                    {
                        // Sprite is visible, so copy the attribute entry over to our
                        // scanline sprite cache. Ive added < 8 here to guard the array
                        // being written to.
                        if self.sprite_count < 8 {
                            // Is this sprite sprite zero?
                            if oam_entry == Addr(0x0000) {
                                // It is, so its possible it may trigger a
                                // sprite zero hit when drawn
                                self.sprite_zero_hit_possible = true;
                            }

                            self.oam
                                .write_entry(oam_entry, self.sprite_scan_line[self.sprite_count]);
                        }
                        self.sprite_count += 1;
                    }
                    oam_entry.inc();
                } // End of sprite evaluation for next scanline

                // Set sprite overflow flag
                self.status.set_sprite_overflow(self.sprite_count >= 8);

                // Now we have an array of the 8 visible sprites for the next scanline. By
                // the nature of this search, they are also ranked in priority, because
                // those lower down in the OAM have the higher priority.

                // We also guarantee that "Sprite Zero" will exist in spriteScanline[0] if
                // it is evaluated to be visible.
            }

            if self.cycle == 340 {
                // Now we're at the very end of the scanline, I'm going to prepare the
                // sprite shifters with the 8 or less selected sprites.

                for i in 0..self.sprite_count {
                    // We need to extract the 8-bit row patterns of the sprite with the
                    // correct vertical offset. The "Sprite Mode" also affects this as
                    // the sprites may be 8 or 16 rows high. Additionally, the sprite
                    // can be flipped both vertically and horizontally. So there's a lot
                    // going on here :P

                    let mut sprite_pattern_bits_lo = Byte(0);
                    let mut sprite_pattern_bits_hi = Byte(0);
                    let mut sprite_pattern_addr_lo = Addr(0);
                    let mut sprite_pattern_addr_hi = Addr(0);

                    // Determine the memory addresses that contain the byte of pattern data. We
                    // only need the lo pattern address, because the hi pattern address is always
                    // offset by 8 from the lo address.
                    if !self.control.sprite_size() {
                        // 8x8 Sprite Mode - The control register determines the pattern table
                        if (self.sprite_scan_line[i].attr & 0x80.into()) == 0x00.into() {
                            // Sprite is NOT flipped vertically, i.e. normal
                            sprite_pattern_addr_lo = (self.control.pattern_sprite().as_addr() << 12)  // Which Pattern Table? 0KB or 4KB offset
                                    | (self.sprite_scan_line[i].id.as_lo_addr() << 4)  // Which Cell? Tile ID * 16 (16 bytes per tile)
                                    | (self.scanline - i16::from(self.sprite_scan_line[i].y)).into();
                        // Which Row in cell? (0->7)
                        } else {
                            // Sprite is flipped vertically, i.e. upside down
                            sprite_pattern_addr_lo = (self.control.pattern_sprite().as_addr() << 12)  // Which Pattern Table? 0KB or 4KB offset
                                    | (self.sprite_scan_line[i].id.as_lo_addr() << 4)  // Which Cell? Tile ID * 16 (16 bytes per tile)
                                    | (7i16 - (self.scanline - i16::from(self.sprite_scan_line[i].y))).into();
                            // Which Row in cell? (7->0)
                        }
                    } else {
                        // 8x16 Sprite Mode - The sprite attribute determines the pattern table
                        if (self.sprite_scan_line[i].attr & Byte(0x80)) == Byte(0x00) {
                            // Sprite is NOT flipped vertically, i.e. normal
                            if self.scanline - i16::from(self.sprite_scan_line[i].y) < 8 {
                                // Reading Top half Tile
                                sprite_pattern_addr_lo = ((self.sprite_scan_line[i].id.as_lo_addr() & Addr(0x0001)) << 12)  // Which Pattern Table? 0KB or 4KB offset
                                        | ((self.sprite_scan_line[i].id.as_lo_addr() & Addr(0x00FE)) << 4)  // Which Cell? Tile ID * 16 (16 bytes per tile)
                                        | ((self.scanline - i16::from(self.sprite_scan_line[i].y)) & 0x07).into();
                            // Which Row in cell? (0->7)
                            } else {
                                // Reading Bottom Half Tile
                                sprite_pattern_addr_lo = ((self.sprite_scan_line[i].id.as_lo_addr() & Addr(0x0001)) << 12)  // Which Pattern Table? 0KB or 4KB offset
                                        | ((self.sprite_scan_line[i].id.as_lo_addr() & Addr(0x00FE) + Addr(0x0001)) << 4)  // Which Cell? Tile ID * 16 (16 bytes per tile)
                                        | ((self.scanline - i16::from(self.sprite_scan_line[i].y)) & 0x07).into();
                                // Which Row in cell? (0->7)
                            }
                        } else {
                            // Sprite is flipped vertically, i.e. upside down
                            if self.scanline - i16::from(self.sprite_scan_line[i].y) < 8 {
                                // Reading Top half Tile
                                sprite_pattern_addr_lo = ((self.sprite_scan_line[i].id.as_lo_addr() & Addr(0x0001)) << 12)    // Which Pattern Table? 0KB or 4KB offset
                                        | ((self.sprite_scan_line[i].id.as_lo_addr() & Addr(0x00FE) + Addr(0x0001)) << 4)    // Which Cell? Tile ID * 16 (16 bytes per tile)
                                        | (7i16 - (self.scanline - i16::from(self.sprite_scan_line[i].y)) & 0x07).into();
                            // Which Row in cell? (0->7)
                            } else {
                                // Reading Bottom Half Tile
                                sprite_pattern_addr_lo = ((self.sprite_scan_line[i].id.as_lo_addr() & Addr(0x0001)) << 12)    // Which Pattern Table? 0KB or 4KB offset
                                        | ((self.sprite_scan_line[i].id.as_lo_addr() & Addr(0x00FE)) << 4)    // Which Cell? Tile ID * 16 (16 bytes per tile)
                                        | (7i16 - (self.scanline - i16::from(self.sprite_scan_line[i].y)) & 0x07).into();
                                // Which Row in cell? (0->7)
                            }
                        }
                    }

                    // Phew... XD I'm absolutely certain you can use some fantastic bit
                    // manipulation to reduce all of that to a few one liners, but in this
                    // form it's easy to see the processes required for the different
                    // sizes and vertical orientations

                    // Hi bit plane equivalent is always offset by 8 bytes from lo bit plane
                    sprite_pattern_addr_hi = sprite_pattern_addr_lo + Addr(8);

                    // Now we have the address of the sprite patterns, we can read them
                    sprite_pattern_bits_lo = self.read_chr(cart, sprite_pattern_addr_lo);
                    sprite_pattern_bits_hi = self.read_chr(cart, sprite_pattern_addr_hi);

                    // If the sprite is flipped horizontally, we need to flip the
                    // pattern bytes.
                    if (self.sprite_scan_line[i].attr & Byte(0x40)) != Byte(0x00) {
                        // This little lambda function "flips" a byte
                        // so 0b11100000 becomes 0b00000111. It's very
                        // clever, and stolen completely from here:
                        // https://stackoverflow.com/a/2602885
                        let flipbyte = |mut b: Byte| {
                            b = (b & Byte(0xF0)) >> 4 | (b & Byte(0x0F)) << 4;
                            b = (b & Byte(0xCC)) >> 2 | (b & Byte(0x33)) << 2;
                            b = (b & Byte(0xAA)) >> 1 | (b & Byte(0x55)) << 1;
                            b
                        };

                        // Flip Patterns Horizontally
                        sprite_pattern_bits_lo = flipbyte(sprite_pattern_bits_lo);
                        sprite_pattern_bits_hi = flipbyte(sprite_pattern_bits_hi);
                    }

                    // Finally! We can load the pattern into our sprite shift registers
                    // ready for rendering on the next scanline
                    self.sprite_shifter_pattern_lo[i] = sprite_pattern_bits_lo;
                    self.sprite_shifter_pattern_hi[i] = sprite_pattern_bits_hi;
                }
            }
        }

        if self.scanline == 240 {
            // Post Render Scanline - Do Nothing!
        }

        if self.scanline >= 241 && self.scanline < 261 {
            if self.scanline == 241 && self.cycle == 1 {
                // Effectively end of frame, so set vertical blank flag
                self.status.enable_vertical_blank();

                // If the control register tells us to emit a NMI when
                // entering vertical blanking period, do it! The CPU
                // will be informed that rendering is complete so it can
                // perform operations with the PPU knowing it wont
                // produce visible artefacts
                if self.control.enable_nmi() {
                    self.nmi = true;
                }
            }
        }

        // Composition - We now have background & foreground pixel information for this cycle

        // Background =============================================================
        let mut bg_pixel = Addr(0x0000); // The 2-bit pixel to be rendered
        let mut bg_palette = Addr(0x0000); // The 3-bit index of the palette the pixel indexes

        // We only render backgrounds if the PPU is enabled to do so. Note if
        // background rendering is disabled, the pixel and palette combine
        // to form 0x00. This will fall through the colour tables to yield
        // the current background colour in effect
        if self.mask.render_background() {
            if self.mask.render_background_left() || (self.cycle >= 9) {
                // Handle Pixel Selection by selecting the relevant bit
                // depending upon fine x scolling. This has the effect of
                // offsetting ALL background rendering by a set number
                // of pixels, permitting smooth scrolling
                let bit_mux = Word(0x8000) >> u16::from(self.fine_x);

                // Select Plane pixels by extracting from the shifter
                // at the required location.
                let p0_pixel = (self.bg_shifter_pattern_lo & bit_mux) > Word(0);
                let p1_pixel = (self.bg_shifter_pattern_hi & bit_mux) > Word(0);

                // Combine to form pixel index
                bg_pixel = (p1_pixel.as_addr() << 1) | p0_pixel.as_addr();

                // Get palette
                let bg_pal0 = (self.bg_shifter_attr_lo & bit_mux) > Word(0);
                let bg_pal1 = (self.bg_shifter_attr_hi & bit_mux) > Word(0);
                bg_palette = (bg_pal1.as_addr() << 1) | bg_pal0.as_addr();
            }
        }

        // Foreground =============================================================
        let mut fg_pixel = Addr(0x0000); // The 2-bit pixel to be rendered
        let mut fg_palette = Addr(0x0000); // The 3-bit index of the palette the pixel indexes
        let mut fg_priority = false; // A bit of the sprite attribute indicates if its
                                     // more important than the background
        if self.mask.render_sprites() {
            // Iterate through all sprites for this scanline. This is to maintain
            // sprite priority. As soon as we find a non transparent pixel of
            // a sprite we can abort
            if self.mask.render_sprites_left() || (self.cycle >= 9) {
                self.sprite_zero_being_rendered = false;

                for i in 0..self.sprite_count {
                    // Scanline cycle has "collided" with sprite, shifters taking over
                    if self.sprite_scan_line[i].x == Byte(0) {
                        // Note Fine X scrolling does not apply to sprites, the game
                        // should maintain their relationship with the background. So
                        // we'll just use the MSB of the shifter

                        // Determine the pixel value...
                        let fg_pixel_lo =
                            (self.sprite_shifter_pattern_lo[i] & Byte(0x80)) > Byte(0);
                        let fg_pixel_hi =
                            (self.sprite_shifter_pattern_hi[i] & Byte(0x80)) > Byte(0);
                        fg_pixel = (fg_pixel_hi.as_addr() << 1) | fg_pixel_lo.as_addr();

                        // Extract the palette from the bottom two bits. Recall
                        // that foreground palettes are the latter 4 in the
                        // palette memory.
                        fg_palette = ((self.sprite_scan_line[i].attr & Byte(0x03)) + Byte(0x04))
                            .as_lo_addr();
                        fg_priority = (self.sprite_scan_line[i].attr & Byte(0x20)) == Byte(0);

                        // If pixel is not transparent, we render it, and dont
                        // bother checking the rest because the earlier sprites
                        // in the list are higher priority
                        if fg_pixel != Addr(0) {
                            if i == 0
                            // Is this sprite zero?
                            {
                                self.sprite_zero_being_rendered = true;
                            }

                            break;
                        }
                    }
                }
            }
        }

        // Now we have a background pixel and a foreground pixel. They need
        // to be combined. It is possible for sprites to go behind background
        // tiles that are not "transparent", yet another neat trick of the PPU
        // that adds complexity for us poor emulator developers...

        let mut pixel = Addr(0x0000); // The FINAL Pixel...
        let mut palette = Addr(0x0000); // The FINAL Palette...

        if bg_pixel == Addr(0) && fg_pixel == Addr(0) {
            // The background pixel is transparent
            // The foreground pixel is transparent
            // No winner, draw "background" colour
            pixel = Addr(0x0000);
            palette = Addr(0x0000);
        } else if bg_pixel == Addr(0) && fg_pixel > Addr(0) {
            // The background pixel is transparent
            // The foreground pixel is visible
            // Foreground wins!
            pixel = fg_pixel;
            palette = fg_palette;
        } else if bg_pixel > Addr(0) && fg_pixel == Addr(0) {
            // The background pixel is visible
            // The foreground pixel is transparent
            // Background wins!
            pixel = bg_pixel;
            palette = bg_palette;
        } else if bg_pixel > Addr(0) && fg_pixel > Addr(0) {
            // The background pixel is visible
            // The foreground pixel is visible
            // Hmmm...
            if fg_priority {
                // Foreground cheats its way to victory!
                pixel = fg_pixel;
                palette = fg_palette;
            } else {
                // Background is considered more important!
                pixel = bg_pixel;
                palette = bg_palette;
            }

            // Sprite Zero Hit detection
            if self.sprite_zero_hit_possible && self.sprite_zero_being_rendered {
                // Sprite zero is a collision between foreground and background
                // so they must both be enabled
                if self.mask.render_background() && self.mask.render_sprites() {
                    // The left edge of the screen has specific switches to control
                    // its appearance. This is used to smooth inconsistencies when
                    // scrolling (since sprites x coord must be >= 0)
                    if !(self.mask.render_background_left() || self.mask.render_sprites_left()) {
                        if self.cycle >= 9 && self.cycle < 258 {
                            self.status.enable_sprite_zero_hit();
                        }
                    } else {
                        if self.cycle >= 1 && self.cycle < 258 {
                            self.status.enable_sprite_zero_hit();
                        }
                    }
                }
            }
        }

        // Now we have a final pixel colour, and a palette for this cycle
        // of the current scanline. Let's at long last, draw that ^&%*er :P
        //sprScreen.SetPixel(cycle - 1, scanline, GetColourFromPaletteRam(palette, pixel));

        // Advance renderer - it never stops, it's relentless
        self.cycle += 1;
        if self.mask.render_background() || self.mask.render_sprites() {
            if self.cycle == 260 && self.scanline < 240 {
                cart.scanline();
            }
        }

        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
                //self.frame_complete = true;
                self.odd_frame = !self.odd_frame;
            }
        }
    }
}
