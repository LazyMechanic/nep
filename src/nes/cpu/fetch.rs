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

pub fn fetch_operand<T, U>(
    cpu_registers: &mut T,
    cpu_bus: &mut U,
    mode: &AddressingMode,
) -> (
    /*       operand*/ Operand,
    /*need add cycle*/ bool,
)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let result = match mode {
        AddressingMode::XXX => unimplemented!(),
        AddressingMode::ACC => (Operand::None, false),
        AddressingMode::ABS => unimplemented!(),
        AddressingMode::ABX => unimplemented!(),
        AddressingMode::ABY => unimplemented!(),
        AddressingMode::IMP => (Operand::None, false),
        AddressingMode::IMM => unimplemented!(),
        AddressingMode::IND => unimplemented!(),
        AddressingMode::IZX => unimplemented!(),
        AddressingMode::IZY => unimplemented!(),
        AddressingMode::REL => fetch_relative(cpu_registers, cpu_bus),
        AddressingMode::ZP0 => unimplemented!(),
        AddressingMode::ZPX => unimplemented!(),
        AddressingMode::ZPY => unimplemented!(),
    };

    result
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

    Word::from_bytes(lo, hi)
}

fn fetch_absolute<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let base = fetch_word(cpu_registers, cpu_bus);
    (Operand::Addr(base.into()), false)
}

fn fetch_relative<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let base = fetch_byte(cpu_registers, cpu_bus);
    if base.is_neg() {
        (Operand::Addr(base.into_lo_addr() | 0xFF00.into()), false)
    } else {
        (Operand::Addr(base.into_lo_addr()), false)
    }
}
