use super::bus::CpuBus;
use super::opcode::OpCode;
use super::operand::Operand;
use super::registers::CpuRegisters;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum AddressingMode {
    XXX, // Unknown mode
    ACC, // Accumulator
    ABS, // Absolute
    ABX, // Absolute with X offset
    ABY, // Absolute with Y offset
    IMP, // Implied
    IMM, // Immediate
    IND, // Indirect
    IZX, // Indirect X
    IZY, // Indirect Y
    REL, // Relative
    ZP0, // Zero page
    ZPX, // Zero page with X offset
    ZPY, // Zero page with Y offset
}

pub fn fetch_instruction_code<T, U>(registers: &mut T, bus: &mut U) -> Byte
where
    T: CpuRegisters,
    U: CpuBus,
{
    fetch_byte(registers, bus)
}

pub fn fetch_operand<T, U>(
    opcode: &OpCode,
    registers: &mut T,
    bus: &mut U,
) -> (
    /*       operand*/ Operand,
    /*need add cycle*/ bool,
)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let result = match opcode.mode {
        AddressingMode::XXX => (Operand::None, false),
        AddressingMode::ACC => fetch_accumulator(registers, bus),
        AddressingMode::ABS => fetch_absolute(registers, bus),
        AddressingMode::ABX => fetch_absolute_x(registers, bus),
        AddressingMode::ABY => fetch_absolute_y(registers, bus),
        AddressingMode::IMP => fetch_implied(registers, bus),
        AddressingMode::IMM => fetch_immediate(registers, bus),
        AddressingMode::IND => fetch_indirect(registers, bus),
        AddressingMode::IZX => fetch_indirect_x(registers, bus),
        AddressingMode::IZY => fetch_indirect_y(registers, bus),
        AddressingMode::REL => fetch_relative(registers, bus),
        AddressingMode::ZP0 => fetch_zero_page(registers, bus),
        AddressingMode::ZPX => fetch_zero_page_x(registers, bus),
        AddressingMode::ZPY => fetch_zero_page_y(registers, bus),
    };

    result
}

fn fetch_byte<T, U>(registers: &mut T, bus: &mut U) -> Byte
where
    T: CpuRegisters,
    U: CpuBus,
{
    let b = bus.read(registers.pc());
    registers.inc_pc();
    b
}

fn fetch_word<T, U>(registers: &mut T, bus: &mut U) -> Word
where
    T: CpuRegisters,
    U: CpuBus,
{
    let lo = fetch_byte(registers, bus);
    let hi = fetch_byte(registers, bus);

    Word::from_bytes(lo, hi)
}

fn fetch_accumulator<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    (Operand::None, false)
}

fn fetch_absolute<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let word = fetch_word(registers, bus);
    (Operand::Addr(word.into()), false)
}

fn fetch_absolute_x<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let word = fetch_word(registers, bus);
    let addr = Addr::from(word.clone()) + registers.x().as_lo_addr();

    if word.hi() != addr.hi() {
        (Operand::Addr(addr), true)
    } else {
        (Operand::Addr(addr), false)
    }
}

fn fetch_absolute_y<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let word = fetch_word(registers, bus);
    let addr = Addr::from(word.clone()) + registers.y().as_lo_addr();

    if word.hi() != addr.hi() {
        (Operand::Addr(addr), true)
    } else {
        (Operand::Addr(addr), false)
    }
}

fn fetch_implied<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    (Operand::None, false)
}

fn fetch_immediate<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let b = fetch_byte(registers, bus);
    (Operand::Byte(b), false)
}

fn fetch_indirect<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let mut word = fetch_word(registers, bus);
    if word.lo().is_set() {
        // Simulate page boundary hardware bug
        let addr =
            bus.read(word.hi_word().into()).as_hi_addr() | bus.read(word.into()).as_lo_addr();
        (Operand::Addr(addr), false)
    } else {
        // Behave normally
        let addr = bus.read(word.inc().into()).as_hi_addr() | bus.read(word.into()).as_lo_addr();
        (Operand::Addr(addr), false)
    }
}

fn fetch_indirect_x<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let mut base = fetch_byte(registers, bus).as_lo_addr() + registers.x().as_lo_addr();

    let lo = bus.read(base);
    let hi = bus.read(base.inc());

    let addr = Addr::from_bytes(lo, hi);

    (Operand::Addr(addr), false)
}

fn fetch_indirect_y<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let mut base = fetch_byte(registers, bus).as_lo_addr();

    let lo = bus.read(base);
    let hi = bus.read(base.inc());

    let addr = Addr::from_bytes(lo, hi) + registers.y().as_lo_addr();

    if addr.hi() != hi {
        (Operand::Addr(addr), true)
    } else {
        (Operand::Addr(addr), false)
    }
}

fn fetch_relative<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let base = fetch_byte(registers, bus);
    if base.is_neg() {
        (Operand::Addr(base.as_lo_addr() | 0xFF00.into()), false)
    } else {
        (Operand::Addr(base.as_lo_addr()), false)
    }
}

fn fetch_zero_page<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let addr = fetch_byte(registers, bus).as_lo_addr();
    (Operand::Addr(addr), false)
}

fn fetch_zero_page_x<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let addr = fetch_byte(registers, bus).as_lo_addr() + registers.x().as_lo_addr();
    (Operand::Addr(addr), false)
}

fn fetch_zero_page_y<T, U>(registers: &mut T, bus: &mut U) -> (Operand, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let addr = fetch_byte(registers, bus).as_lo_addr() + registers.y().as_lo_addr();
    (Operand::Addr(addr), false)
}
