pub mod cartridge;
pub mod clock;
pub mod cpu;
pub mod dma;
pub mod ppu;
pub mod prelude;
pub mod ram;
pub mod types;
pub mod utils;

use prelude::*;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub fn run<T>(file_path: T) -> Result<()>
where
    T: AsRef<Path>,
{
    let mut cart = Rc::new(RefCell::new(cartridge::Cartridge::from_file(file_path)?));

    let mut internal_ram = Rc::new(RefCell::new(ram::internal_ram::InternalRam::new()));

    let mut dma = dma::Dma::new();
    let mut cpu = cpu::Cpu::default();
    let mut ppu = Rc::new(RefCell::new(ppu::Ppu::new(cart.clone())));

    let mut cpu_bus = cpu::bus::Bus::new(cart.clone(), internal_ram.clone(), ppu.clone());

    cpu.reset(&mut cpu_bus);
    ppu.borrow_mut().reset(cart.clone());

    let mut clock = clock::Clock::default();
    loop {
        clock.update();

        if clock.need_step_cpu() {
            if dma.should_run() {
                dma.step(&mut cpu_bus);
            } else {
                cpu.step(&mut cpu_bus);
            }
        }

        if clock.need_step_ppu() {
            ppu.borrow_mut().step();
        }
    }

    Ok(())
}
