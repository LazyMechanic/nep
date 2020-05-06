use super::bus::CpuBus;
use super::registers::CpuRegisters;
use crate::types::*;

#[derive(Debug, Clone)]
pub enum Instruction {
    ILLEGAL, // Illegal operation
    ADC,     // Add with Carry In
    AND,     // Bitwise Logic AND
    ASL,     // Arithmetic Shift Left
    BCC,     // Branch if Carry Clear
    BCS,     // Branch if Carry Set
    BEQ,     // Branch if Equal
    BIT,     // Bit test
    BMI,     // Branch if Negative
    BNE,     // Branch if Not Equal
    BPL,     // Branch if Positive
    BRK,     // Break
    BVC,     // Branch if Overflow Clear
    BVS,     // Branch if Overflow Set
    CLC,     // Clear Carry Flag
    CLD,     // Clear Decimal Flag
    CLI,     // Disable Interrupts / Clear Interrupt Flag
    CLV,     // Clear Overflow Flag
    CMP,     // Compare Accumulator
    CPX,     // Compare X Register
    CPY,     // Compare Y Register
    DEC,     // Decrement Value at Memory Location
    DEX,     // Decrement X Register
    DEY,     // Decrement Y Register
    EOR,     // Bitwise Logic XOR
    INC,     // Increment Value at Memory Location
    INX,     // Increment X Register
    INY,     // Increment Y Register
    JMP,     // Jump To Location
    JSR,     // Jump To Sub-Routine
    LDA,     // Load The Accumulator
    LDX,     // Load The X Register
    LDY,     // Load The Y Register
    LSR,     // Logical shift right
    NOP,     // No operation
    ORA,     // Bitwise Logic OR
    PHA,     // Push Accumulator to Stack
    PHP,     // Push Status Register to Stack
    PLA,     // Pop Accumulator off Stack
    PLP,     // Pop Status Register off Stack
    ROL,     // Rotate One Bit Left
    ROR,     // Rotate One Bit Right
    RTI,     // Return from Interrupt
    RTS,     // Return from Subroutine
    SBC,     // Subtraction with Borrow In
    SEC,     // Set Carry Flag
    SED,     // Set Decimal Flag
    SEI,     // Set Interrupt Flag / Enable Interrupts
    STA,     // Store Accumulator at Address
    STX,     // Store X Register at Address
    STY,     // Store Y Register at Address
    TAX,     // Transfer Accumulator to X Register
    TAY,     // Transfer Accumulator to Y Register
    TSX,     // Transfer Stack Pointer to X Register
    TXA,     // Transfer X Register to Accumulator
    TXS,     // Transfer X Register to Stack Pointer
    TYA,     // Transfer Y Register to Accumulator
}

pub fn adc<T, U>(cpu_registers: &mut T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn and<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn asl<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn bcc<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn bcs<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn beq<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn bit<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn bmi<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn bne<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn bpl<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn brk<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn bvc<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn bvs<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn clc<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn cld<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn cli<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn clv<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn cmp<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn cpx<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn cpy<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn dec<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn dex<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn dey<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn eor<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn inc<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn inx<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn iny<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn jmp<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn jsr<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn lda<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn ldx<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn ldy<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn lsr<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn nop<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn ora<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn pha<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn php<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn pla<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn plp<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn rol<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn ror<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn rti<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn rts<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn sbc<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn sec<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn sed<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn sei<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn sta<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn stx<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn sty<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn tax<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn tay<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn tsx<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn txa<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn txs<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn tya<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

pub fn illegal<T, U>(cpu_registers: &T, cpu_bus: &mut U, operand: Word) -> NumOfCycles
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}
