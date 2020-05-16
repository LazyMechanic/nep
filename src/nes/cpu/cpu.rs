use super::addressing::{self, AddressingMode};
use super::bus::CpuBus;
use super::instruction;
use super::opcode::{self, OpCode};
use super::registers::Registers;
use crate::prelude::*;

pub struct Cpu {
    regs: Registers,
    cycles: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            cycles: 0,
        }
    }

    // Forces the 6502 into a known state. This is hard-wired inside the CPU. The
    // registers are set to 0x00, the status register is cleared except for unused
    // bit which remains at 1. An absolute address is read from location 0xFFFC
    // which contains a second address that the program counter is set to. This
    // allows the programmer to jump to a known and programmable location in the
    // memory to start executing from.
    pub fn reset(&mut self, mut bus: CpuBus) {
        self.regs.reset(bus);
        self.cycles = 8;
    }

    fn push(&mut self, bus: &mut CpuBus, v: Byte) {
        let addr = self.regs.sp().as_lo_addr() | 0x0100.into();
        bus.write(addr, v);
        self.regs.dec_sp();
    }

    // Interrupt requests are a complex operation and only happen if the
    // "disable interrupt" flag is 0. IRQs can happen at any time, but
    // you dont want them to be destructive to the operation of the running
    // program. Therefore the current instruction is allowed to finish
    // (which I facilitate by doing the whole thing when cycles == 0) and
    // then the current program counter is stored on the stack. Then the
    // current status register is stored on the stack. When the routine
    // that services the interrupt has finished, the status register
    // and program counter can be restored to how they where before it
    // occurred. This is impemented by the "RTI" instruction. Once the IRQ
    // has happened, in a similar way to a reset, a programmable address
    // is read form hard coded location 0xFFFE, which is subsequently
    // set to the program counter.
    pub fn irq(&mut self, mut bus: CpuBus) {
        if !self.regs.interrupt() {
            // Push the program counter to the stack. It's 16-bits dont
            // forget so that takes two pushes
            self.push(&mut bus, self.regs.pc().hi());
            self.push(&mut bus, self.regs.pc().lo());

            // Then Push the status register to the stack
            self.regs.set_break_mode(false);
            self.regs.set_reserved(true);
            self.regs.set_interrupt(true);

            self.push(&mut bus, self.regs.status());

            // Read new program counter location from fixed address
            let mut addr: Addr = 0xFFFE.into();
            let lo = bus.read(addr);
            let hi = bus.read(addr.inc());

            let pc = Addr::from_bytes(lo, hi);
            self.regs.set_pc(pc);

            // IRQs take time
            self.cycles = 7;
        }
    }

    // A Non-Maskable Interrupt cannot be ignored. It behaves in exactly the
    // same way as a regular IRQ, but reads the new program counter address
    // form location 0xFFFA.
    pub fn nmi(&mut self, mut bus: CpuBus) {
        // Push the program counter to the stack. It's 16-bits dont
        // forget so that takes two pushes
        self.push(&mut bus, self.regs.pc().hi());
        self.push(&mut bus, self.regs.pc().lo());

        // Then Push the status register to the stack
        self.regs.set_break_mode(false);
        self.regs.set_reserved(true);
        self.regs.set_interrupt(true);

        self.push(&mut bus, self.regs.status());

        // Read new program counter location from fixed address
        let mut addr: Addr = 0xFFFA.into();
        let lo = bus.read(addr);
        let hi = bus.read(addr.inc());

        let pc = Addr::from_bytes(lo, hi);
        self.regs.set_pc(pc);

        // NMI take time
        self.cycles = 8;
    }

    pub fn step(&mut self, mut bus: CpuBus) {
        // Each instruction requires a variable number of clock cycles to execute.
        // In my emulation, I only care about the final result and so I perform
        // the entire computation in one hit. In hardware, each clock cycle would
        // perform "microcode" style transformations of the CPUs state.
        //
        // To remain compliant with connected devices, it's important that the
        // emulation also takes "time" in order to execute instructions, so I
        // implement that delay by simply counting down the cycles required by
        // the instruction. When it reaches 0, the instruction is complete, and
        // the next one is ready to be executed.
        if self.cycles == 0 {
            let code = addressing::fetch_instruction_code(&mut self.regs, &mut bus);

            // Always set the unused status flag bit to 1
            self.regs.set_reserved(true);

            let opcodes = &opcode::OPCODES;
            let opcode = &opcodes[code.0 as usize];

            // Get Starting number of cycles
            self.cycles = opcode.cycles;

            let (operand, addr_need_add) =
                addressing::fetch_operand(&opcode, &mut self.regs, &mut bus);

            let (additional_cycle, inst_need_add) =
                instruction::exec_instruction(&opcode, &mut self.regs, &mut bus, operand);

            self.cycles += additional_cycle;

            // The addressmode and opcode may have altered the number
            // of cycles this instruction requires before its completed
            if addr_need_add && inst_need_add {
                self.cycles += 1;
            }

            // Always set the unused status flag bit to 1
            self.regs.set_reserved(true);
        }

        self.cycles -= 1;
    }
}
