use super::addressing::AddressingMode;
use super::bus::CpuBus;
use super::operand::Operand;
use super::registers::CpuRegisters;
use crate::types::*;

pub fn fetch_instruction_code<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> Byte
where
    T: CpuRegisters,
    U: CpuBus,
{
    fetch_byte(cpu_registers, cpu_bus)
}

pub fn fetch_operand<T, U>(cpu_registers: &mut T, cpu_bus: &mut U, mode: &AddressingMode) -> Operand
where
    T: CpuRegisters,
    U: CpuBus,
{
    let operand: Operand = match mode {
        AddressingMode::XXX => unimplemented!(),
        AddressingMode::ACC => unimplemented!(),
        AddressingMode::ABS => unimplemented!(),
        AddressingMode::ABX => unimplemented!(),
        AddressingMode::ABY => unimplemented!(),
        AddressingMode::IMP => unimplemented!(),
        AddressingMode::IMM => unimplemented!(),
        AddressingMode::IND => unimplemented!(),
        AddressingMode::IZX => unimplemented!(),
        AddressingMode::IZY => unimplemented!(),
        AddressingMode::REL => unimplemented!(),
        AddressingMode::ZP0 => unimplemented!(),
        AddressingMode::ZPX => unimplemented!(),
        AddressingMode::ZPY => unimplemented!(),
    };

    operand
}

fn fetch_byte<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> Byte
where
    T: CpuRegisters,
    U: CpuBus,
{
    let b = cpu_bus.read(cpu_registers.get_pc());
    cpu_registers.inc_pc();
    b
}

fn fetch_word<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> Word
where
    T: CpuRegisters,
    U: CpuBus,
{
    let lo = fetch_byte(cpu_registers, cpu_bus);
    let hi = fetch_byte(cpu_registers, cpu_bus);

    Word::from_bytes(hi, lo)
}
