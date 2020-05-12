use crate::cpu::registers::Registers;

#[derive(Default)]
pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    // Forces the 6502 into a known state. This is hard-wired inside the CPU. The
    // registers are set to 0x00, the status register is cleared except for unused
    // bit which remains at 1. An absolute address is read from location 0xFFFC
    // which contains a second address that the program counter is set to. This
    // allows the programmer to jump to a known and programmable location in the
    // memory to start executing from.
    pub fn reset(&mut self) {}

    pub fn irq(&mut self) {}

    pub fn nmi(&mut self) {}

    pub fn step(&mut self) {}
}
