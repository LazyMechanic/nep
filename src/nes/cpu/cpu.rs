use super::addressing::{self, AddressingMode};
use super::bus::{Bus, CpuBus};
use super::instruction;
use super::opcode::{self, OpCode};
use super::registers::{CpuRegisters, Registers};

#[derive(Default)]
pub struct Cpu {
    regs:   Registers,
    cycles: u8,
}

impl Cpu {
    // Forces the 6502 into a known state. This is hard-wired inside the CPU. The
    // registers are set to 0x00, the status register is cleared except for unused
    // bit which remains at 1. An absolute address is read from location 0xFFFC
    // which contains a second address that the program counter is set to. This
    // allows the programmer to jump to a known and programmable location in the
    // memory to start executing from.
    pub fn reset<T>(&mut self, bus: &mut T)
    where
        T: CpuBus,
    {
        self.regs.reset(bus);
    }

    pub fn irq<T>(&mut self, bus: &mut T)
    where
        T: CpuBus,
    {
    }

    pub fn nmi<T>(&mut self, bus: &mut T)
    where
        T: CpuBus,
    {
    }

    pub fn step<T>(&mut self, bus: &mut T)
    where
        T: CpuBus,
    {
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
            let code = addressing::fetch_instruction_code(&mut self.regs, bus);

            // Always set the unused status flag bit to 1
            self.regs.set_reserved(true);

            let opcodes = &opcode::OPCODES;
            let opcode = &opcodes[code.0 as usize];

            // Get Starting number of cycles
            self.cycles = opcode.cycles;

            let (operand, addr_need_add) = addressing::fetch_operand(&opcode, &mut self.regs, bus);

            let (additional_cycle, intr_need_add) =
                instruction::exec_instruction(&opcode, &mut self.regs, bus, operand);

            self.cycles += additional_cycle;

            // The addressmode and opcode may have altered the number
            // of cycles this instruction requires before its completed
            if addr_need_add && intr_need_add {
                self.cycles += 1;
            }

            // Always set the unused status flag bit to 1
            self.regs.set_reserved(true);
        }

        self.cycles -= 1;
    }
}
