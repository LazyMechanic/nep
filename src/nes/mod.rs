pub mod cartridge;
pub mod clock;
pub mod cpu;
pub mod ppu;
pub mod prelude;
pub mod ram;
pub mod types;
pub mod utils;

use prelude::*;
use std::path::Path;

const RAM_SIZE: usize = 8192; // 8 kb

pub fn run<T>(file_path: T) -> Result<()>
where
    T: AsRef<Path>,
{
    let mut cart = cartridge::Cartridge::from_file(file_path)?;
    let mut ram = ram::Ram::with_size(RAM_SIZE);

    let mut bus = cpu::bus::Bus::new(&mut ram, &mut cart);

    let mut cpu = cpu::Cpu::default();
    let mut ppu = ppu::Ppu::default();

    cpu.reset(&mut bus);
    ppu.reset();

    let mut clock = clock::Clock::default();
    loop {
        clock.update();

        if clock.need_step_cpu() {
            // TODO: cpu.step(&mut bus);
        }

        if clock.need_step_ppu() {
            // TODO: ppu.step();
        }
    }

    Ok(())
}
