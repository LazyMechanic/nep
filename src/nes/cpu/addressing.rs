use super::bus::CpuBus;
use super::opcode::OpCode;
use super::operand::Operand;
use super::registers::Registers;
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

pub fn fetch_instruction_code(registers: &mut Registers, bus: &mut CpuBus) -> Byte {
    fetch_byte(registers, bus)
}

pub fn fetch_operand(
    opcode: &OpCode,
    registers: &mut Registers,
    bus: &mut CpuBus,
) -> (
    /*       operand*/ Operand,
    /*need add cycle*/ bool,
) {
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

fn fetch_byte(registers: &mut Registers, bus: &mut CpuBus) -> Byte {
    let b = bus.read(registers.pc());
    registers.inc_pc();
    b
}

fn fetch_word(registers: &mut Registers, bus: &mut CpuBus) -> Word {
    let lo = fetch_byte(registers, bus);
    let hi = fetch_byte(registers, bus);

    Word::from_bytes(lo, hi)
}

fn fetch_accumulator(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    (Operand::None, false)
}

fn fetch_absolute(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let word = fetch_word(registers, bus);
    (Operand::Addr(word.into()), false)
}

fn fetch_absolute_x(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let word = fetch_word(registers, bus);
    let addr = Addr::from(word).overflowing_add(registers.x().as_lo_addr());

    if word.hi() != addr.hi() {
        (Operand::Addr(addr), true)
    } else {
        (Operand::Addr(addr), false)
    }
}

fn fetch_absolute_y(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let word = fetch_word(registers, bus);
    let addr = Addr::from(word).overflowing_add(registers.y().as_lo_addr());

    if word.hi() != addr.hi() {
        (Operand::Addr(addr), true)
    } else {
        (Operand::Addr(addr), false)
    }
}

fn fetch_implied(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let b = registers.a();
    (Operand::Byte(b), false)
}

fn fetch_immediate(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let b = fetch_byte(registers, bus);
    (Operand::Byte(b), false)
}

fn fetch_indirect(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let mut word = fetch_word(registers, bus);
    if word.lo().is_set() {
        // Simulate page boundary hardware bug
        let lo = bus.read(word.into());
        let hi = bus.read(word.hi_word().into());

        let addr = Addr::from_bytes(lo, hi);
        (Operand::Addr(addr), false)
    } else {
        // Behave normally
        let lo = bus.read(word.into());
        let hi = bus.read(Addr::from(word) + Addr(1));

        let addr = Addr::from_bytes(lo, hi);
        (Operand::Addr(addr), false)
    }
}

fn fetch_indirect_x(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let mut base = fetch_byte(registers, bus)
        .as_lo_addr()
        .overflowing_add(registers.x().as_lo_addr());

    let lo = bus.read(base.lo_addr());
    let hi = bus.read(base.inc().lo_addr());

    let addr = Addr::from_bytes(lo, hi);

    (Operand::Addr(addr), false)
}

fn fetch_indirect_y(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let mut base = fetch_byte(registers, bus);

    let lo = bus.read(base.as_lo_addr());
    let hi = bus.read(base.inc().as_lo_addr());

    let addr = Addr::from_bytes(lo, hi).overflowing_add(registers.y().as_lo_addr());

    if addr.hi() != hi {
        (Operand::Addr(addr), true)
    } else {
        (Operand::Addr(addr), false)
    }
}

fn fetch_relative(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let base = fetch_byte(registers, bus);

    if base.is_neg() {
        let addr = (base.as_lo_addr() | Addr(0xFF00)).overflowing_add(registers.pc());
        (Operand::Addr(addr), false)
    } else {
        let addr = base.as_lo_addr().overflowing_add(registers.pc());
        (Operand::Addr(addr), false)
    }
}

fn fetch_zero_page(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let addr = fetch_byte(registers, bus).as_lo_addr();
    (Operand::Addr(addr), false)
}

fn fetch_zero_page_x(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let mut addr = fetch_byte(registers, bus).as_lo_addr() + registers.x().as_lo_addr();
    addr = addr.lo_addr();
    (Operand::Addr(addr), false)
}

fn fetch_zero_page_y(registers: &mut Registers, bus: &mut CpuBus) -> (Operand, bool) {
    let mut addr = fetch_byte(registers, bus).as_lo_addr() + registers.y().as_lo_addr();
    addr = addr.lo_addr();
    (Operand::Addr(addr), false)
}
