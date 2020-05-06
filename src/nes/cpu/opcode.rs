use super::addressing::AddressingMode;
use super::instruction::Instruction;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct OpCode {
    pub name:  Instruction,
    pub mode:  AddressingMode,
    pub cycle: NumOfCycles,
}
