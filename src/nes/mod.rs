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
use ppu::Ppu;
use ram::Ram;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub fn run<T>(file_path: T) -> Result<()>
where
    T: AsRef<Path>,
{
    let mut cart = Cartridge::from_file(file_path)?;

    let mut ram = Ram::new();
    let mut dma = Dma::new();

    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();

    cpu.reset(CpuBus::new(&mut cart, &mut ram, &mut ppu, &mut dma));
    ppu.reset();

    let mut clock = Clock::new();
    loop {
        clock.update();

        if clock.need_step_ppu() {
            ppu.step(&mut cart);
        }

        if clock.need_step_cpu() {
            // Is the system performing a DMA transfer form CPU memory to
            // OAM memory on PPU?...
            if dma.has_request() {
                // ...Yes! We need to wait until the next even CPU clock cycle
                // before it starts...
                if dma.wait_start() {
                    // ...So hang around in here each clock until 1 or 2 cycles
                    // have elapsed...
                    if clock.need_start_dma() {
                        // ...and finally allow DMA to start
                        dma.start();
                    }
                } else {
                    // Step DMA transfer
                    dma.step(&mut ram, ppu.oam_mut());
                }
            } else {
                cpu.step(CpuBus::new(&mut cart, &mut ram, &mut ppu, &mut dma));
            }
        }
    }

    Ok(())
}
