use super::addressing::AddressingMode;
use super::bus::CpuBus;
use super::opcode::OpCode;
use super::operand::Operand;
use super::registers::CpuRegisters;
use crate::prelude::*;

pub type NumOfCycles = u8;

#[derive(Debug, Clone)]
pub enum Instruction {
    XXX, // Illegal operation
    ADC, // Add with Carry In
    AND, // Bitwise Logic AND
    ASL, // Arithmetic Shift Left
    BCC, // Branch if Carry Clear
    BCS, // Branch if Carry Set
    BEQ, // Branch if Equal
    BIT, // Bit test
    BMI, // Branch if Negative
    BNE, // Branch if Not Equal
    BPL, // Branch if Positive
    BRK, // Break
    BVC, // Branch if Overflow Clear
    BVS, // Branch if Overflow Set
    CLC, // Clear Carry Flag
    CLD, // Clear Decimal Flag
    CLI, // Disable Interrupts / Clear Interrupt Flag
    CLV, // Clear Overflow Flag
    CMP, // Compare Accumulator
    CPX, // Compare X Register
    CPY, // Compare Y Register
    DEC, // Decrement Value at Memory Location
    DEX, // Decrement X Register
    DEY, // Decrement Y Register
    EOR, // Bitwise Logic XOR
    INC, // Increment Value at Memory Location
    INX, // Increment X Register
    INY, // Increment Y Register
    JMP, // Jump To Location
    JSR, // Jump To Sub-Routine
    LDA, // Load The Accumulator
    LDX, // Load The X Register
    LDY, // Load The Y Register
    LSR, // Logical shift right
    NOP, // No operation
    ORA, // Bitwise Logic OR
    PHA, // Push Accumulator to Stack
    PHP, // Push Status Register to Stack
    PLA, // Pop Accumulator off Stack
    PLP, // Pop Status Register off Stack
    ROL, // Rotate One Bit Left
    ROR, // Rotate One Bit Right
    RTI, // Return from Interrupt
    RTS, // Return from Subroutine
    SBC, // Subtraction with Borrow In
    SEC, // Set Carry Flag
    SED, // Set Decimal Flag
    SEI, // Set Interrupt Flag / Enable Interrupts
    STA, // Store Accumulator at Address
    STX, // Store X Register at Address
    STY, // Store Y Register at Address
    TAX, // Transfer Accumulator to X Register
    TAY, // Transfer Accumulator to Y Register
    TSX, // Transfer Stack Pointer to X Register
    TXA, // Transfer X Register to Accumulator
    TXS, // Transfer X Register to Stack Pointer
    TYA, // Transfer Y Register to Accumulator
}

pub fn exec_instruction<T, U>(
    opcode: &OpCode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (
    /*          additional cycles*/ NumOfCycles,
    /*need add cycle by addr mode*/ bool,
)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let result = match opcode.inst {
        Instruction::XXX => xxx(&opcode.mode, registers, bus, operand),
        Instruction::ADC => adc(&opcode.mode, registers, bus, operand),
        Instruction::AND => and(&opcode.mode, registers, bus, operand),
        Instruction::ASL => asl(&opcode.mode, registers, bus, operand),
        Instruction::BCC => bcc(&opcode.mode, registers, bus, operand),
        Instruction::BCS => bcs(&opcode.mode, registers, bus, operand),
        Instruction::BEQ => beq(&opcode.mode, registers, bus, operand),
        Instruction::BIT => bit(&opcode.mode, registers, bus, operand),
        Instruction::BMI => bmi(&opcode.mode, registers, bus, operand),
        Instruction::BNE => bne(&opcode.mode, registers, bus, operand),
        Instruction::BPL => bpl(&opcode.mode, registers, bus, operand),
        Instruction::BRK => brk(&opcode.mode, registers, bus, operand),
        Instruction::BVC => bvc(&opcode.mode, registers, bus, operand),
        Instruction::BVS => bvs(&opcode.mode, registers, bus, operand),
        Instruction::CLC => clc(&opcode.mode, registers, bus, operand),
        Instruction::CLD => cld(&opcode.mode, registers, bus, operand),
        Instruction::CLI => cli(&opcode.mode, registers, bus, operand),
        Instruction::CLV => clv(&opcode.mode, registers, bus, operand),
        Instruction::CMP => cmp(&opcode.mode, registers, bus, operand),
        Instruction::CPX => cpx(&opcode.mode, registers, bus, operand),
        Instruction::CPY => cpy(&opcode.mode, registers, bus, operand),
        Instruction::DEC => dec(&opcode.mode, registers, bus, operand),
        Instruction::DEX => dex(&opcode.mode, registers, bus, operand),
        Instruction::DEY => dey(&opcode.mode, registers, bus, operand),
        Instruction::EOR => eor(&opcode.mode, registers, bus, operand),
        Instruction::INC => inc(&opcode.mode, registers, bus, operand),
        Instruction::INX => inx(&opcode.mode, registers, bus, operand),
        Instruction::INY => iny(&opcode.mode, registers, bus, operand),
        Instruction::JMP => jmp(&opcode.mode, registers, bus, operand),
        Instruction::JSR => jsr(&opcode.mode, registers, bus, operand),
        Instruction::LDA => lda(&opcode.mode, registers, bus, operand),
        Instruction::LDX => ldx(&opcode.mode, registers, bus, operand),
        Instruction::LDY => ldy(&opcode.mode, registers, bus, operand),
        Instruction::LSR => lsr(&opcode.mode, registers, bus, operand),
        Instruction::NOP => nop(&opcode.mode, registers, bus, operand),
        Instruction::ORA => ora(&opcode.mode, registers, bus, operand),
        Instruction::PHA => pha(&opcode.mode, registers, bus, operand),
        Instruction::PHP => php(&opcode.mode, registers, bus, operand),
        Instruction::PLA => pla(&opcode.mode, registers, bus, operand),
        Instruction::PLP => plp(&opcode.mode, registers, bus, operand),
        Instruction::ROL => rol(&opcode.mode, registers, bus, operand),
        Instruction::ROR => ror(&opcode.mode, registers, bus, operand),
        Instruction::RTI => rti(&opcode.mode, registers, bus, operand),
        Instruction::RTS => rts(&opcode.mode, registers, bus, operand),
        Instruction::SBC => sbc(&opcode.mode, registers, bus, operand),
        Instruction::SEC => sec(&opcode.mode, registers, bus, operand),
        Instruction::SED => sed(&opcode.mode, registers, bus, operand),
        Instruction::SEI => sei(&opcode.mode, registers, bus, operand),
        Instruction::STA => sta(&opcode.mode, registers, bus, operand),
        Instruction::STX => stx(&opcode.mode, registers, bus, operand),
        Instruction::STY => sty(&opcode.mode, registers, bus, operand),
        Instruction::TAX => tax(&opcode.mode, registers, bus, operand),
        Instruction::TAY => tay(&opcode.mode, registers, bus, operand),
        Instruction::TSX => tsx(&opcode.mode, registers, bus, operand),
        Instruction::TXA => txa(&opcode.mode, registers, bus, operand),
        Instruction::TXS => txs(&opcode.mode, registers, bus, operand),
        Instruction::TYA => tya(&opcode.mode, registers, bus, operand),
    };

    result
}

fn unwrap_operand<T>(bus: &mut T, operand: Operand) -> Byte
where
    T: CpuBus,
{
    match operand {
        Operand::None => Byte(0),
        Operand::Byte(v) => v,
        Operand::Addr(v) => bus.read(v),
    }
}

fn unwrap_operand_with_addr<T>(bus: &mut T, operand: Operand) -> (Byte, Addr)
where
    T: CpuBus,
{
    match operand {
        Operand::None => (Byte(0), Addr(0)),
        Operand::Byte(v) => (v, Addr(0)),
        Operand::Addr(v) => (bus.read(v), v),
    }
}

fn jump_to<T>(registers: &mut T, addr: Addr)
where
    T: CpuRegisters,
{
    registers.set_pc(addr);
}

fn is_same_page(left: Addr, right: Addr) -> bool {
    left.hi() == right.hi()
}

// Instruction: Add with Carry In
// Function:    A = A + M + C
// Flags Out:   C, V, N, Z
//
// Explanation:
// The purpose of this function is to add a value to the accumulator and a carry bit. If
// the result is > 255 there is an overflow setting the carry bit. Ths allows you to
// chain together ADC instructions to add numbers larger than 8-bits. This in itself is
// simple, however the 6502 supports the concepts of Negativity/Positivity and Signed Overflow.
//
// 10000100 = 128 + 4 = 132 in normal circumstances, we know this as unsigned and it allows
// us to represent numbers between 0 and 255 (given 8 bits). The 6502 can also interpret
// this word as something else if we assume those 8 bits represent the range -128 to +127,
// i.e. it has become signed.
//
// Since 132 > 127, it effectively wraps around, through -128, to -124. This wraparound is
// called overflow, and this is a useful to know as it indicates that the calculation has
// gone outside the permissable range, and therefore no longer makes numeric sense.
//
// Note the implementation of ADD is the same in binary, this is just about how the numbers
// are represented, so the word 10000100 can be both -124 and 132 depending upon the
// context the programming is using it in. We can prove this!
//
//  10000100 =  132  or  -124
// +00010001 = + 17      + 17
//  ========    ===       ===     See, both are valid additions, but our interpretation of
//  10010101 =  149  or  -107     the context changes the value, not the hardware!
//
// In principle under the -128 to 127 range:
// 10000000 = -128, 11111111 = -1, 00000000 = 0, 00000000 = +1, 01111111 = +127
// therefore negative numbers have the most significant set, positive numbers do not
//
// To assist us, the 6502 can set the overflow flag, if the result of the addition has
// wrapped around. V <- ~(A^M) & A^(A+M+C) :D lol, let's work out why!
//
// Let's suppose we have A = 30, M = 10 and C = 0
//          A = 30 = 00011110
//          M = 10 = 00001010+
//     RESULT = 40 = 00101000
//
// Here we have not gone out of range. The resulting significant bit has not changed.
// So let's make a truth table to understand when overflow has occurred. Here I take
// the MSB of each component, where R is RESULT.
//
// A  M  R | V | A^R | A^M |~(A^M) |
// 0  0  0 | 0 |  0  |  0  |   1   |
// 0  0  1 | 1 |  1  |  0  |   1   |
// 0  1  0 | 0 |  0  |  1  |   0   |
// 0  1  1 | 0 |  1  |  1  |   0   |  so V = ~(A^M) & (A^R)
// 1  0  0 | 0 |  1  |  1  |   0   |
// 1  0  1 | 0 |  0  |  1  |   0   |
// 1  1  0 | 1 |  1  |  0  |   1   |
// 1  1  1 | 0 |  0  |  0  |   1   |
//
// We can see how the above equation calculates V, based on A, M and R. V was chosen
// based on the following hypothesis:
//       Positive Number + Positive Number = Negative Result -> Overflow
//       Negative Number + Negative Number = Positive Result -> Overflow
//       Positive Number + Negative Number = Either Result -> Cannot Overflow
//       Positive Number + Positive Number = Positive Result -> OK! No Overflow
//       Negative Number + Negative Number = Negative Result -> OK! NO Overflow
fn adc<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let fetched = unwrap_operand(bus, operand);
    let acc = registers.get_a();
    let carry = registers.get_carry();

    let res = fetched.as_lo_word() + acc.as_lo_word() + carry.as_word();

    registers
        .set_overflow(!(acc ^ fetched).is_neg() && (acc ^ res.lo()).is_neg())
        .update_negative_by(res.lo())
        .update_zero_by(res.lo())
        .set_carry(res > 0x00FF.into())
        .set_a(res.lo());

    (0, true)
}

// OK! Complicated operations are done! the following are much simpler
// and conventional. The typical order of events is:
// 1) Fetch the data you are working with
// 2) Perform calculation
// 3) Store the result in desired place
// 4) Set Flags of the status register
// 5) Return if instruction has potential to require additional
//    clock cycle
//
// Instruction: Bitwise Logic AND
// Function:    A = A & M
// Flags Out:   N, Z
fn and<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let fetched = unwrap_operand(bus, operand);
    let acc = registers.get_a();
    let res = fetched & acc;

    registers
        .set_a(res)
        .set_zero(res.is_clear())
        .set_negative(res.is_neg());

    (0, true)
}

// Instruction: Arithmetic Shift Left
// Function:    A = C <- (A << 1) <- 0
// Flags Out:   N, Z, C
fn asl<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let (fetched, addr) = unwrap_operand_with_addr(bus, operand);
    let res = fetched.into_lo_word() << 1;

    registers
        .set_carry(res.hi() > 0x00.into())
        .set_zero(res.lo().is_clear())
        .set_negative(res.lo().is_neg());

    match mode {
        AddressingMode::ACC => {
            registers.set_a(res.lo());
        }
        _ => {
            bus.write(addr, res.lo());
        }
    }

    (0, false)
}

// Instruction: Branch if Carry Clear
// Function:    if(C == 0) pc = address
fn bcc<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    if !registers.get_carry() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.get_pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

// Instruction: Branch if Carry Set
// Function:    if(C == 1) pc = address
fn bcs<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    if registers.get_carry() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.get_pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

// Instruction: Branch if Equal
// Function:    if(Z == 1) pc = address
fn beq<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    if registers.get_zero() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.get_pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

fn bit<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    let fetched = unwrap_operand(bus, operand);
    let acc = registers.get_a();

    registers
        .update_negative_by(fetched)
        .update_zero_by(fetched & acc)
        .set_overflow(fetched.inspect_bit(6));

    (0, false)
}

// Instruction: Branch if Negative
// Function:    if(N == 1) pc = address
fn bmi<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    if registers.get_negative() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.get_pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

fn bne<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    if !registers.get_zero() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.get_pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

// Instruction: Branch if Positive
// Function:    if(N == 0) pc = address
fn bpl<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    if !registers.get_negative() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.get_pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

fn brk<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn bvc<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn bvs<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn clc<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn cld<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn cli<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn clv<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn cmp<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn cpx<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn cpy<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn dec<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn dex<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn dey<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn eor<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn inc<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn inx<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn iny<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn jmp<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn jsr<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn lda<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn ldx<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn ldy<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn lsr<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn nop<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn ora<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn pha<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn php<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn pla<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn plp<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn rol<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn ror<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn rti<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn rts<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn sbc<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn sec<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn sed<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn sei<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn sta<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn stx<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn sty<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn tax<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn tay<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn tsx<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn txa<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn txs<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn tya<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}

fn xxx<T, U>(
    mode: &AddressingMode,
    registers: &mut T,
    bus: &mut U,
    operand: Operand,
) -> (NumOfCycles, bool)
where
    T: CpuRegisters,
    U: CpuBus,
{
    unimplemented!();
}
