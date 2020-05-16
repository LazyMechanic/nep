pub mod cartridge;
pub mod clock;
pub mod cpu;
pub mod dma;
pub mod joypad;
pub mod ppu;
pub mod prelude;
pub mod ram;
pub mod types;
pub mod utils;

use prelude::*;

use cartridge::Cartridge;
use clock::Clock;
use cpu::bus::CpuBus;
use cpu::Cpu;
use dma::Dma;
use joypad::Joypad;
use ppu::Ppu;
use ram::Ram;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub struct Emu {
    clock: Clock,
    cart:  Cartridge,
    ram:   Ram,
    dma:   Dma,
    cpu:   Cpu,
    ppu:   Ppu,
    joy_1: Joypad,
    joy_2: Joypad,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            clock: Clock::new(),
            cart:  Cartridge::new(),
            ram:   Ram::new(),
            dma:   Dma::new(),
            cpu:   Cpu::new(),
            ppu:   Ppu::new(),
            joy_1: Joypad::default(),
            joy_2: Joypad::default(),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset(CpuBus::new(
            &mut self.cart,
            &mut self.ram,
            &mut self.ppu,
            &mut self.dma,
            &mut self.joy_1,
            &mut self.joy_2,
        ));
        self.ppu.reset();
        self.clock.reset();
    }

    pub fn load<P>(&mut self, file_path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.cart.load(file_path)?;
        self.reset();
        Ok(())
    }

    pub fn step(&mut self, joy_1_state: u8, joy_2_state: u8) {
        self.joy_1 = joy_1_state.into();
        self.joy_2 = joy_2_state.into();

        self.clock.update();

        if self.clock.need_step_ppu() {
            self.ppu.step(&mut self.cart);
        }

        if self.clock.need_step_cpu() {
            // Is the system performing a DMA transfer form CPU memory to
            // OAM memory on PPU?...
            if self.dma.has_request() {
                // ...Yes! We need to wait until the next even CPU clock cycle
                // before it starts...
                if self.dma.wait_start() {
                    // ...So hang around in here each clock until 1 or 2 cycles
                    // have elapsed...
                    if self.clock.need_start_dma() {
                        // ...and finally allow DMA to start
                        self.dma.start();
                    }
                } else {
                    // Step DMA transfer
                    self.dma.step(&mut self.ram, self.ppu.oam_mut());
                }
            } else {
                self.cpu.step(CpuBus::new(
                    &mut self.cart,
                    &mut self.ram,
                    &mut self.ppu,
                    &mut self.dma,
                    &mut self.joy_1,
                    &mut self.joy_2,
                ));
            }
        }
    }
}
