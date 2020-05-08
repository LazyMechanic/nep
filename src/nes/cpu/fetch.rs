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
        AddressingMode::XXX => (Operand::None, false),
        AddressingMode::ACC => fetch_accumulator(cpu_registers, cpu_bus),
        AddressingMode::ABS => fetch_absolute(cpu_registers, cpu_bus),
        AddressingMode::ABX => fetch_absolute_x(cpu_registers, cpu_bus),
        AddressingMode::ABY => fetch_absolute_y(cpu_registers, cpu_bus),
        AddressingMode::IMP => fetch_implied(cpu_registers, cpu_bus),
        AddressingMode::IMM => fetch_immediate(cpu_registers, cpu_bus),
        AddressingMode::IND => fetch_indirect(cpu_registers, cpu_bus),
        AddressingMode::IZX => fetch_indirect_x(cpu_registers, cpu_bus),
        AddressingMode::IZY => fetch_indirect_y(cpu_registers, cpu_bus),
        AddressingMode::REL => fetch_relative(cpu_registers, cpu_bus),
        AddressingMode::ZP0 => fetch_zero_page(cpu_registers, cpu_bus),
        AddressingMode::ZPX => fetch_zero_page_x(cpu_registers, cpu_bus),
        AddressingMode::ZPY => fetch_zero_page_y(cpu_registers, cpu_bus),
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

fn fetch_accumulator<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    (Operand::None, false)
}

fn fetch_absolute<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let word = fetch_word(cpu_registers, cpu_bus);
    (Operand::Addr(word.into()), false)
}

fn fetch_absolute_x<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let word = fetch_word(cpu_registers, cpu_bus);
    let result = word + cpu_registers.get_x().into_lo_word();

    if word.hi() != result.hi() {
        (Operand::Addr(result.into()), true)
    } else {
        (Operand::Addr(result.into()), false)
    }
}

fn fetch_absolute_y<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let word = fetch_word(cpu_registers, cpu_bus);
    let addr = Addr::from(word.clone()) + cpu_registers.get_y().into_lo_addr();

    if word.hi() != addr.hi() {
        (Operand::Addr(addr), true)
    } else {
        (Operand::Addr(addr), false)
    }
}

fn fetch_implied<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    (Operand::None, false)
}

fn fetch_immediate<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let b = fetch_byte(cpu_registers, cpu_bus);
    (Operand::Byte(b), false)
}

fn fetch_indirect<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let mut word = fetch_word(cpu_registers, cpu_bus);
    if word.lo().is_set() {
        // Simulate page boundary hardware bug
        let addr = cpu_bus.read(word.hi_word().into()).into_hi_addr()
            | cpu_bus.read(word.into()).into_lo_addr();
        (Operand::Addr(addr), false)
    } else {
        // Behave normally
        let addr = cpu_bus.read(word.inc().into()).into_hi_addr()
            | cpu_bus.read(word.into()).into_lo_addr();
        (Operand::Addr(addr), false)
    }
}

fn fetch_indirect_x<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let mut base =
        fetch_byte(cpu_registers, cpu_bus).into_lo_addr() + cpu_registers.get_x().into_lo_addr();

    let lo = cpu_bus.read(base);
    let hi = cpu_bus.read(*base.inc());

    let addr = Addr::from_bytes(lo, hi);

    (Operand::Addr(addr), false)
}

fn fetch_indirect_y<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let mut base = fetch_byte(cpu_registers, cpu_bus).into_lo_addr();

    let lo = cpu_bus.read(base);
    let hi = cpu_bus.read(*base.inc());

    let addr = Addr::from_bytes(lo, hi) + cpu_registers.get_x().into_lo_addr();

    if addr.hi() != hi {
        (Operand::Addr(addr), true)
    } else {
        (Operand::Addr(addr), false)
    }
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

fn fetch_zero_page<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let addr = fetch_byte(cpu_registers, cpu_bus).into_lo_addr();
    (Operand::Addr(addr), false)
}

fn fetch_zero_page_x<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let addr =
        fetch_byte(cpu_registers, cpu_bus).into_lo_addr() + cpu_registers.get_x().into_lo_addr();
    (Operand::Addr(addr), false)
}

fn fetch_zero_page_y<T, U>(cpu_registers: &mut T, cpu_bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let addr =
        fetch_byte(cpu_registers, cpu_bus).into_lo_addr() + cpu_registers.get_y().into_lo_addr();
    (Operand::Addr(addr), false)
}
