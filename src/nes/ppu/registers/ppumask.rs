use crate::prelude::*;
use bitfield::*;

bitfield! {
    #[derive(Default, Clone, Copy)]
    pub struct PpuMask(u8);
    impl Debug;
    // ==================================================== Bit position: 7654 3210
    pub bool, grayscale,              set_grayscale:              0; // 0b0000_000*
    pub bool, render_background_left, set_render_background_left: 1; // 0b0000_00*0
    pub bool, render_sprites_left,    set_render_sprites_left:    2; // 0b0000_0*00
    pub bool, render_background,      set_render_background:      3; // 0b0000_*000
    pub bool, render_sprites,         set_render_sprites:         4; // 0b000*_0000
    pub bool, enhance_red,            set_enhance_red:            5; // 0b00*0_0000
    pub bool, enhance_green,          set_enhance_green:          6; // 0b0*00_0000
    pub bool, enhance_blue,           set_enhance_blue:           7; // 0b*000_0000
}

impl PpuMask {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn enable_grayscale(&mut self) {
        self.set_grayscale(true);
    }

    pub fn enable_render_background_left(&mut self) {
        self.set_render_background_left(true);
    }

    pub fn enable_render_sprites_left(&mut self) {
        self.set_render_sprites_left(true);
    }

    pub fn enable_render_background(&mut self) {
        self.set_render_background(true);
    }

    pub fn enable_render_sprites(&mut self) {
        self.set_render_sprites(true);
    }

    pub fn enable_enhance_red(&mut self) {
        self.set_enhance_red(true);
    }

    pub fn enable_enhance_green(&mut self) {
        self.set_enhance_green(true);
    }

    pub fn enable_enhance_blue(&mut self) {
        self.set_enhance_blue(true);
    }

    pub fn disable_grayscale(&mut self) {
        self.set_grayscale(false);
    }

    pub fn disable_render_background_left(&mut self) {
        self.set_render_background_left(false);
    }

    pub fn disable_render_sprites_left(&mut self) {
        self.set_render_sprites_left(false);
    }

    pub fn disable_render_background(&mut self) {
        self.set_render_background(false);
    }

    pub fn disable_render_sprites(&mut self) {
        self.set_render_sprites(false);
    }

    pub fn disable_enhance_red(&mut self) {
        self.set_enhance_red(false);
    }

    pub fn disable_enhance_green(&mut self) {
        self.set_enhance_green(false);
    }

    pub fn disable_enhance_blue(&mut self) {
        self.set_enhance_blue(false);
    }
}

impl From<u8> for PpuMask {
    fn from(v: u8) -> Self {
        Self(v)
    }
}

impl From<PpuMask> for u8 {
    fn from(v: PpuMask) -> Self {
        v.0
    }
}

impl From<Byte> for PpuMask {
    fn from(v: Byte) -> Self {
        Self(v.into())
    }
}

impl From<PpuMask> for Byte {
    fn from(v: PpuMask) -> Self {
        Self(v.0)
    }
}
