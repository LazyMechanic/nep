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
use joypad::JoypadState;
use ppu::screen::Screen;
use ppu::Ppu;
use ram::Ram;

use std::cell::RefCell;
use std::io::{Read, Seek};
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
            joy_1: Joypad::new(),
            joy_2: Joypad::new(),
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

    pub fn screen(&self) -> &Screen {
        self.ppu.screen()
    }

    pub fn load<F: Read + Seek>(&mut self, file: &mut F) -> Result<()> {
        self.cart.load(file)?;
        self.reset();
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<()> {
        self.cart.load_from_file(file_path)?;
        self.reset();
        Ok(())
    }

    pub fn update_joypads(&mut self, joy_1_state: u8, joy_2_state: u8) {
        self.joy_1.update(JoypadState(joy_1_state));
        self.joy_2.update(JoypadState(joy_2_state));
    }

    pub fn step(&mut self) {
        loop {
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

            // The PPU is capable of emitting an interrupt to indicate the
            // vertical blanking period has been entered. If it has, we need
            // to send that irq to the CPU.
            if self.ppu.has_nmi() {
                self.ppu.clear_nmi();
                self.cpu.nmi(CpuBus::new(
                    &mut self.cart,
                    &mut self.ram,
                    &mut self.ppu,
                    &mut self.dma,
                    &mut self.joy_1,
                    &mut self.joy_2,
                ));
            }

            // Check if cartridge is requesting IRQ
            if self.cart.has_irq() {
                self.cart.clear_irq();
                self.cpu.irq(CpuBus::new(
                    &mut self.cart,
                    &mut self.ram,
                    &mut self.ppu,
                    &mut self.dma,
                    &mut self.joy_1,
                    &mut self.joy_2,
                ));
            }

            self.clock.update();

            if self.ppu.screen().ready {
                break;
            }
        }
    }
}
