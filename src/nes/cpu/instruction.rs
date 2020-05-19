use super::addressing::AddressingMode;
use super::bus::CpuBus;
use super::opcode::OpCode;
use super::operand::Operand;
use super::registers::Registers;
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

pub fn exec_instruction(
    opcode: &OpCode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (
    /*          additional cycles*/ NumOfCycles,
    /*need add cycle by addr mode*/ bool,
) {
    // println!("opcode: {:?}", opcode);
    // println!("operand: {:?}", operand);
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

fn unwrap_operand(bus: &mut CpuBus, operand: Operand) -> Byte {
    match operand {
        Operand::None => panic!("expected Operand::Byte || Operand::Addr, Operand::None handled"),
        Operand::Byte(v) => v,
        Operand::Addr(v) => bus.read(v),
    }
}

fn unwrap_operand_with_addr(bus: &mut CpuBus, operand: Operand) -> (Byte, Addr) {
    match operand {
        Operand::None => panic!("expected Operand::Byte || Operand::Addr, Operand::None handled"),
        Operand::Byte(v) => (v, Addr(0)),
        Operand::Addr(v) => (bus.read(v), v),
    }
}

fn jump_to(registers: &mut Registers, addr: Addr) {
    registers.set_pc(addr);
}

fn is_same_page(left: Addr, right: Addr) -> bool {
    left.hi() == right.hi()
}

fn push(registers: &mut Registers, bus: &mut CpuBus, v: Byte) {
    let addr = registers.sp().as_lo_addr() | Addr(0x0100);
    bus.write(addr, v);
    registers.dec_sp();
}

fn push_pc(registers: &mut Registers, bus: &mut CpuBus) {
    let pc = registers.pc();
    push(registers, bus, pc.hi());
    push(registers, bus, pc.lo());
}

fn push_status(registers: &mut Registers, bus: &mut CpuBus) {
    let status = registers.status();
    push(registers, bus, status);
}

fn pop(registers: &mut Registers, bus: &mut CpuBus) -> Byte {
    registers.inc_sp();
    let addr = registers.sp().as_lo_addr() | Addr(0x0100);
    bus.read(addr)
}

fn pop_pc(registers: &mut Registers, bus: &mut CpuBus) {
    let lo = pop(registers, bus);
    let hi = pop(registers, bus);
    let addr = Addr::from_bytes(lo, hi);
    registers.set_pc(addr);
}

fn pop_status(registers: &mut Registers, bus: &mut CpuBus) {
    let status = pop(registers, bus);
    registers.set_status(status);
    registers.set_reserved(true);
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
fn adc(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand).as_lo_word();
    let acc = registers.a().as_lo_word();
    let carry = registers.carry().as_word();

    let res = fetched + acc + carry;
    let overflow = (!(acc ^ fetched) & (acc ^ res)) & Word(0x0080) != Word(0x0000);

    registers
        .set_overflow(overflow)
        .update_negative_by(res.lo())
        .update_zero_by(res.lo())
        .set_carry(res > Word(0x00FF))
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
fn and(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);
    let acc = registers.a();
    let res = fetched & acc;

    registers
        .set_a(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, true)
}

// Instruction: Arithmetic Shift Left
// Function:    A = C <- (A << 1) <- 0
// Flags Out:   N, Z, C
fn asl(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let (fetched, addr) = unwrap_operand_with_addr(bus, operand);
    let res = fetched.as_lo_word() << 1;

    registers
        .set_carry(res.hi() > Byte(0x00))
        .update_zero_by(res.lo())
        .update_negative_by(res.lo());

    match mode {
        AddressingMode::IMP => {
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
fn bcc(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    if !registers.carry() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.pc()) {
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
fn bcs(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    if registers.carry() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.pc()) {
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
fn beq(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    if registers.zero() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

fn bit(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);
    let acc = registers.a();
    let res = fetched & acc;

    registers
        .update_negative_by(fetched)
        .update_zero_by(res)
        .set_overflow(fetched.inspect_bit(6));

    (0, false)
}

// Instruction: Branch if Negative
// Function:    if(N == 1) pc = address
fn bmi(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    if registers.negative() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

// Instruction: Branch if Not Equal
// Function:    if(Z == 0) pc = address
fn bne(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    if !registers.zero() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.pc()) {
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
fn bpl(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    if !registers.negative() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

// Instruction: Break
// Function:    Program Sourced Interrupt
fn brk(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    registers.inc_pc();

    registers.set_interrupt(true);
    push_pc(registers, bus);

    registers.set_break_mode(true);
    push_status(registers, bus);
    registers.set_break_mode(false);

    let lo = bus.read(Addr(0xFFFE));
    let hi = bus.read(Addr(0xFFFF));
    let pc = Addr::from_bytes(lo, hi);
    registers.set_pc(pc);

    (0, false)
}

// Instruction: Branch if Overflow Clear
// Function:    if(V == 0) pc = address
fn bvc(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    if !registers.overflow() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

// Instruction: Branch if Overflow Set
// Function:    if(V == 1) pc = address
fn bvs(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    if registers.overflow() {
        let mut additional_cycles: NumOfCycles = 1;
        let addr = operand.unwrap_addr();

        if !is_same_page(addr, registers.pc()) {
            additional_cycles += 1;
        }

        jump_to(registers, addr);

        (additional_cycles, false)
    } else {
        (0, false)
    }
}

// Instruction: Clear Carry Flag
// Function:    C = 0
fn clc(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    registers.set_carry(false);
    (0, false)
}

// Instruction: Clear Decimal Flag
// Function:    D = 0
fn cld(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    registers.set_decimal_mode(false);
    (0, false)
}

// Instruction: Disable Interrupts / Clear Interrupt Flag
// Function:    I = 0
fn cli(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    registers.set_interrupt(false);
    (0, false)
}

// Instruction: Clear Overflow Flag
// Function:    V = 0
fn clv(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    registers.set_overflow(false);
    (0, false)
}

// Instruction: Compare Accumulator
// Function:    C <- A >= M      Z <- (A - M) == 0
// Flags Out:   N, C, Z
fn cmp(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);
    let reg = registers.a();
    let res = reg.as_lo_word().overflowing_sub(fetched.as_lo_word());

    registers
        .set_carry(reg >= fetched)
        .update_zero_by(res.lo())
        .update_negative_by(res.lo());

    (0, true)
}

// Instruction: Compare X Register
// Function:    C <- X >= M      Z <- (X - M) == 0
// Flags Out:   N, C, Z
fn cpx(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);
    let reg = registers.x();
    let res = reg.as_lo_word().overflowing_sub(fetched.as_lo_word());

    registers
        .set_carry(reg >= fetched)
        .update_zero_by(res.lo())
        .update_negative_by(res.lo());

    (0, true)
}

// Instruction: Compare Y Register
// Function:    C <- Y >= M      Z <- (Y - M) == 0
// Flags Out:   N, C, Z
fn cpy(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);
    let reg = registers.y();
    let res = reg.as_lo_word().overflowing_sub(fetched.as_lo_word());

    registers
        .set_carry(reg >= fetched)
        .update_zero_by(res.lo())
        .update_negative_by(res.lo());

    (0, true)
}

// Instruction: Decrement Value at Memory Location
// Function:    M = M - 1
// Flags Out:   N, Z
fn dec(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let (mut fetched, addr) = unwrap_operand_with_addr(bus, operand);
    let res = fetched.dec();

    bus.write(addr, res);
    registers.update_zero_by(res).update_negative_by(res);

    (0, false)
}

// Instruction: Decrement X Register
// Function:    X = X - 1
// Flags Out:   N, Z
fn dex(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.x().dec();

    registers
        .set_x(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

// Instruction: Decrement Y Register
// Function:    Y = Y - 1
// Flags Out:   N, Z
fn dey(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.y().dec();

    registers
        .set_y(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

// Instruction: Bitwise Logic XOR
// Function:    A = A xor M
// Flags Out:   N, Z
fn eor(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);
    let acc = registers.a();
    let res = acc ^ fetched;

    registers
        .set_a(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, true)
}

// Instruction: Increment Value at Memory Location
// Function:    M = M + 1
// Flags Out:   N, Z
fn inc(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let (mut fetched, addr) = unwrap_operand_with_addr(bus, operand);
    let res = fetched.inc();

    bus.write(addr, res);
    registers.update_zero_by(res).update_negative_by(res);

    (0, false)
}

// Instruction: Increment X Register
// Function:    X = X + 1
// Flags Out:   N, Z
fn inx(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.x().inc();

    registers
        .set_x(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

fn iny(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.y().inc();

    registers
        .set_y(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

// Instruction: Jump To Location
// Function:    pc = address
fn jmp(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let addr = operand.unwrap_addr();
    jump_to(registers, addr);

    (0, false)
}

// Instruction: Jump To Sub-Routine
// Function:    Push current pc to stack, pc = address
fn jsr(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let addr = operand.unwrap_addr();

    registers.dec_pc();
    push_pc(registers, bus);

    registers.set_pc(addr);

    (0, false)
}

// Instruction: Load The Accumulator
// Function:    A = M
// Flags Out:   N, Z
fn lda(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);

    registers
        .set_a(fetched)
        .update_zero_by(fetched)
        .update_negative_by(fetched);

    (0, true)
}

// Instruction: Load The X Register
// Function:    X = M
// Flags Out:   N, Z
fn ldx(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);

    registers
        .set_x(fetched)
        .update_zero_by(fetched)
        .update_negative_by(fetched);

    (0, true)
}

// Instruction: Load The Y Register
// Function:    Y = M
// Flags Out:   N, Z
fn ldy(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);

    registers
        .set_y(fetched)
        .update_zero_by(fetched)
        .update_negative_by(fetched);

    (0, true)
}

fn lsr(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let (fetched, addr) = unwrap_operand_with_addr(bus, operand);
    let carry = fetched.inspect_bit(0);
    let res = fetched >> 1;

    registers
        .set_carry(carry)
        .update_zero_by(res)
        .update_negative_by(res);

    match mode {
        AddressingMode::IMP => {
            registers.set_a(res);
        }
        _ => {
            bus.write(addr, res);
        }
    }

    (0, false)
}

fn nop(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    return match mode {
        AddressingMode::ABX => (0, true),
        _ => (0, false),
    };
}

// Instruction: Bitwise Logic OR
// Function:    A = A | M
// Flags Out:   N, Z
fn ora(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand);
    let acc = registers.a();
    let res = acc | fetched;

    registers
        .set_a(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, true)
}

// Instruction: Push Accumulator to Stack
// Function:    A -> stack
fn pha(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    push(registers, bus, registers.a());
    (0, false)
}

// Instruction: Push Status Register to Stack
// Function:    status -> stack
// Note:        Break flag is set to 1 before push
fn php(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    registers.set_break_mode(true).set_reserved(true);

    push_status(registers, bus);

    registers.set_break_mode(false).set_reserved(false);

    (0, false)
}

// Instruction: Pop Accumulator off Stack
// Function:    A <- stack
// Flags Out:   N, Z
fn pla(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = pop(registers, bus);

    registers
        .set_a(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

// Instruction: Pop Status Register off Stack
// Function:    Status <- stack
fn plp(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    pop_status(registers, bus);
    (0, false)
}

fn rol(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let (fetched, addr) = unwrap_operand_with_addr(bus, operand);
    let carry = registers.carry();
    let res = (fetched.as_lo_word() << 1) | carry.as_word();

    registers
        .set_carry(res.hi() != Byte(0x00))
        .update_zero_by(res.lo())
        .update_negative_by(res.lo());

    match mode {
        AddressingMode::IMP => {
            registers.set_a(res.lo());
        }
        _ => {
            bus.write(addr, res.lo());
        }
    }

    (0, false)
}

fn ror(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let (fetched, addr) = unwrap_operand_with_addr(bus, operand);
    let carry = registers.carry();
    let res = (fetched.as_lo_word() >> 1) | (carry.as_word() << 7);

    registers
        .set_carry(fetched != Byte(0x00))
        .update_zero_by(res.lo())
        .update_negative_by(res.lo());

    match mode {
        AddressingMode::IMP => {
            registers.set_a(res.lo());
        }
        _ => {
            bus.write(addr, res.lo());
        }
    }

    (0, false)
}

fn rti(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    pop_status(registers, bus);
    registers.set_break_mode(false).set_reserved(false);

    pop_pc(registers, bus);

    (0, false)
}

fn rts(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    pop_pc(registers, bus);
    registers.inc_pc();

    (0, false)
}

// Instruction: Subtraction with Borrow In
// Function:    A = A - M - (1 - C)
// Flags Out:   C, V, N, Z
//
// Explanation:
// Given the explanation for ADC above, we can reorganise our data
// to use the same computation for addition, for subtraction by multiplying
// the data by -1, i.e. make it negative
//
// A = A - M - (1 - C)  ->  A = A + -1 * (M - (1 - C))  ->  A = A + (-M + 1 + C)
//
// To make a signed positive number negative, we can invert the bits and add 1
// (OK, I lied, a little bit of 1 and 2s complement :P)
//
//  5 = 00000101
// -5 = 11111010 + 00000001 = 11111011 (or 251 in our 0 to 255 range)
//
// The range is actually unimportant, because if I take the value 15, and add 251
// to it, given we wrap around at 256, the result is 10, so it has effectively
// subtracted 5, which was the original intention. (15 + 251) % 256 = 10
//
// Note that the equation above used (1-C), but this got converted to + 1 + C.
// This means we already have the +1, so all we need to do is invert the bits
// of M, the data(!) therfore we can simply add, exactly the same way we did
// before.
fn sbc(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let fetched = unwrap_operand(bus, operand).as_lo_word();
    let acc = registers.a().as_lo_word();
    let carry = registers.carry().as_word();
    // We can invert the bottom 8 bits with bitwise xor
    let val = fetched ^ Word(0x00FF);
    let res = acc + val + carry;
    // Magic....
    let overflow = (res ^ acc) & (res ^ val) & Word(0x0080) != Word(0x0000);

    registers
        .set_carry(res.hi() != Byte(0x00))
        .set_overflow(overflow)
        .update_zero_by(res.lo())
        .update_negative_by(res.lo())
        .set_a(res.lo());

    (0, true)
}

// Instruction: Set Carry Flag
// Function:    C = 1
fn sec(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    registers.set_carry(true);
    (0, false)
}

// Instruction: Set Decimal Flag
// Function:    D = 1
fn sed(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    registers.set_decimal_mode(true);
    (0, false)
}

// Instruction: Set Interrupt Flag / Enable Interrupts
// Function:    I = 1
fn sei(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    registers.set_interrupt(true);
    (0, false)
}

// Instruction: Store Accumulator at Address
// Function:    M = A
fn sta(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let addr = operand.unwrap_addr();
    let res = registers.a();
    bus.write(addr, res);

    (0, false)
}

// Instruction: Store X Register at Address
// Function:    M = X
fn stx(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let addr = operand.unwrap_addr();
    let res = registers.x();
    bus.write(addr, res);

    (0, false)
}

// Instruction: Store Y Register at Address
// Function:    M = Y
fn sty(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let addr = operand.unwrap_addr();
    let res = registers.y();
    bus.write(addr, res);

    (0, false)
}

// Instruction: Transfer Accumulator to X Register
// Function:    X = A
// Flags Out:   N, Z
fn tax(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.a();

    registers
        .set_x(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

// Instruction: Transfer Accumulator to Y Register
// Function:    Y = A
// Flags Out:   N, Z
fn tay(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.a();

    registers
        .set_y(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

// Instruction: Transfer Stack Pointer to X Register
// Function:    X = stack pointer
// Flags Out:   N, Z
fn tsx(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.sp();

    registers
        .set_x(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

// Instruction: Transfer X Register to Accumulator
// Function:    A = X
// Flags Out:   N, Z
fn txa(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.x();

    registers
        .set_a(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

// Instruction: Transfer X Register to Stack Pointer
// Function:    stack pointer = X
fn txs(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.x();

    registers.set_sp(res);

    (0, false)
}

// Instruction: Transfer Y Register to Accumulator
// Function:    A = Y
// Flags Out:   N, Z
fn tya(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    let res = registers.y();

    registers
        .set_a(res)
        .update_zero_by(res)
        .update_negative_by(res);

    (0, false)
}

// This function captures illegal opcodes
fn xxx(
    mode: &AddressingMode,
    registers: &mut Registers,
    bus: &mut CpuBus,
    operand: Operand,
) -> (NumOfCycles, bool) {
    (0, false)
}
