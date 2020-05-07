use super::bus::CpuBus;
use super::registers::CpuRegisters;
use crate::types::*;

pub fn fetch_instruction_code<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> Byte
where
    T: CpuRegisters,
    U: CpuBus,
{
    let code = cpu_bus.read(cpu_registers.get_pc());
    cpu_registers.inc_pc();
    code
}
