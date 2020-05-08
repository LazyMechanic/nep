use super::addressing::AddressingMode;
use super::instruction::Instruction;
use super::instruction::NumOfCycles;
use crate::prelude::*;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct OpCode {
    pub inst:  Instruction,
    pub mode:  AddressingMode,
    pub cycle: NumOfCycles,
}

impl OpCode {
    pub fn new(inst: Instruction, mode: AddressingMode, cycle: NumOfCycles) -> OpCode {
        OpCode { inst, mode, cycle }
    }
}

type I = Instruction;
type A = AddressingMode;

lazy_static! {
    pub static ref OPCODES: [OpCode; 256] =
    [
        OpCode::new(/*0x00*/ I::BRK, A::IMM, 7),OpCode::new(/*0x01*/ I::ORA, A::IZX, 6),OpCode::new(/*0x02*/ I::XXX, A::IMP, 2),OpCode::new(/*0x03*/ I::XXX, A::IMP, 8),OpCode::new(/*0x04*/ I::NOP, A::IMP, 3),OpCode::new(/*0x05*/ I::ORA, A::ZP0, 3),OpCode::new(/*0x06*/ I::ASL, A::ZP0, 5),OpCode::new(/*0x07*/ I::XXX, A::IMP, 5),OpCode::new(/*0x08*/ I::PHP, A::IMP, 3),OpCode::new(/*0x09*/ I::ORA, A::IMM, 2),OpCode::new(/*0x0A*/ I::ASL, A::ACC, 2),OpCode::new(/*0x0B*/ I::XXX, A::IMP, 2),OpCode::new(/*0x0C*/ I::NOP, A::IMP, 4),OpCode::new(/*0x0D*/ I::ORA, A::ABS, 4),OpCode::new(/*0x0E*/ I::ASL, A::ABS, 6),OpCode::new(/*0x0F*/ I::XXX, A::IMP, 6),
        OpCode::new(/*0x10*/ I::BPL, A::REL, 2),OpCode::new(/*0x11*/ I::ORA, A::IZY, 5),OpCode::new(/*0x12*/ I::XXX, A::IMP, 2),OpCode::new(/*0x13*/ I::XXX, A::IMP, 8),OpCode::new(/*0x14*/ I::NOP, A::IMP, 4),OpCode::new(/*0x15*/ I::ORA, A::ZPX, 4),OpCode::new(/*0x16*/ I::ASL, A::ZPX, 6),OpCode::new(/*0x17*/ I::XXX, A::IMP, 6),OpCode::new(/*0x18*/ I::CLC, A::IMP, 2),OpCode::new(/*0x19*/ I::ORA, A::ABY, 4),OpCode::new(/*0x1A*/ I::NOP, A::IMP, 2),OpCode::new(/*0x1B*/ I::XXX, A::IMP, 7),OpCode::new(/*0x1C*/ I::NOP, A::IMP, 4),OpCode::new(/*0x1D*/ I::ORA, A::ABX, 4),OpCode::new(/*0x1E*/ I::ASL, A::ABX, 7),OpCode::new(/*0x1F*/ I::XXX, A::IMP, 7),
        OpCode::new(/*0x20*/ I::JSR, A::ABS, 6),OpCode::new(/*0x21*/ I::AND, A::IZX, 6),OpCode::new(/*0x22*/ I::XXX, A::IMP, 2),OpCode::new(/*0x23*/ I::XXX, A::IMP, 8),OpCode::new(/*0x24*/ I::BIT, A::ZP0, 3),OpCode::new(/*0x25*/ I::AND, A::ZP0, 3),OpCode::new(/*0x26*/ I::ROL, A::ZP0, 5),OpCode::new(/*0x27*/ I::XXX, A::IMP, 5),OpCode::new(/*0x28*/ I::PLP, A::IMP, 4),OpCode::new(/*0x29*/ I::AND, A::IMM, 2),OpCode::new(/*0x2A*/ I::ROL, A::ACC, 2),OpCode::new(/*0x2B*/ I::XXX, A::IMP, 2),OpCode::new(/*0x2C*/ I::BIT, A::ABS, 4),OpCode::new(/*0x2D*/ I::AND, A::ABS, 4),OpCode::new(/*0x2E*/ I::ROL, A::ABS, 6),OpCode::new(/*0x2F*/ I::XXX, A::IMP, 6),
        OpCode::new(/*0x30*/ I::BMI, A::REL, 2),OpCode::new(/*0x31*/ I::AND, A::IZY, 5),OpCode::new(/*0x32*/ I::XXX, A::IMP, 2),OpCode::new(/*0x33*/ I::XXX, A::IMP, 8),OpCode::new(/*0x34*/ I::NOP, A::IMP, 4),OpCode::new(/*0x35*/ I::AND, A::ZPX, 4),OpCode::new(/*0x36*/ I::ROL, A::ZPX, 6),OpCode::new(/*0x37*/ I::XXX, A::IMP, 6),OpCode::new(/*0x38*/ I::SEC, A::IMP, 2),OpCode::new(/*0x39*/ I::AND, A::ABY, 4),OpCode::new(/*0x3A*/ I::NOP, A::IMP, 2),OpCode::new(/*0x3B*/ I::XXX, A::IMP, 7),OpCode::new(/*0x3C*/ I::NOP, A::IMP, 4),OpCode::new(/*0x3D*/ I::AND, A::ABX, 4),OpCode::new(/*0x3E*/ I::ROL, A::ABX, 7),OpCode::new(/*0x3F*/ I::XXX, A::IMP, 7),
        OpCode::new(/*0x40*/ I::RTI, A::IMP, 6),OpCode::new(/*0x41*/ I::EOR, A::IZX, 6),OpCode::new(/*0x42*/ I::XXX, A::IMP, 2),OpCode::new(/*0x43*/ I::XXX, A::IMP, 8),OpCode::new(/*0x44*/ I::NOP, A::IMP, 3),OpCode::new(/*0x45*/ I::EOR, A::ZP0, 3),OpCode::new(/*0x46*/ I::LSR, A::ZP0, 5),OpCode::new(/*0x47*/ I::XXX, A::IMP, 5),OpCode::new(/*0x48*/ I::PHA, A::IMP, 3),OpCode::new(/*0x49*/ I::EOR, A::IMM, 2),OpCode::new(/*0x4A*/ I::LSR, A::ACC, 2),OpCode::new(/*0x4B*/ I::XXX, A::IMP, 2),OpCode::new(/*0x4C*/ I::JMP, A::ABS, 3),OpCode::new(/*0x4D*/ I::EOR, A::ABS, 4),OpCode::new(/*0x4E*/ I::LSR, A::ABS, 6),OpCode::new(/*0x4F*/ I::XXX, A::IMP, 6),
        OpCode::new(/*0x50*/ I::BVC, A::REL, 2),OpCode::new(/*0x51*/ I::EOR, A::IZY, 5),OpCode::new(/*0x52*/ I::XXX, A::IMP, 2),OpCode::new(/*0x53*/ I::XXX, A::IMP, 8),OpCode::new(/*0x54*/ I::NOP, A::IMP, 4),OpCode::new(/*0x55*/ I::EOR, A::ZPX, 4),OpCode::new(/*0x56*/ I::LSR, A::ZPX, 6),OpCode::new(/*0x57*/ I::XXX, A::IMP, 6),OpCode::new(/*0x58*/ I::CLI, A::IMP, 2),OpCode::new(/*0x59*/ I::EOR, A::ABY, 4),OpCode::new(/*0x5A*/ I::NOP, A::IMP, 2),OpCode::new(/*0x5B*/ I::XXX, A::IMP, 7),OpCode::new(/*0x5C*/ I::NOP, A::IMP, 4),OpCode::new(/*0x5D*/ I::EOR, A::ABX, 4),OpCode::new(/*0x5E*/ I::LSR, A::ABX, 7),OpCode::new(/*0x5F*/ I::XXX, A::IMP, 7),
        OpCode::new(/*0x60*/ I::RTS, A::IMP, 6),OpCode::new(/*0x61*/ I::ADC, A::IZX, 6),OpCode::new(/*0x62*/ I::XXX, A::IMP, 2),OpCode::new(/*0x63*/ I::XXX, A::IMP, 8),OpCode::new(/*0x64*/ I::NOP, A::IMP, 3),OpCode::new(/*0x65*/ I::ADC, A::ZP0, 3),OpCode::new(/*0x66*/ I::ROR, A::ZP0, 5),OpCode::new(/*0x67*/ I::XXX, A::IMP, 5),OpCode::new(/*0x68*/ I::PLA, A::IMP, 4),OpCode::new(/*0x69*/ I::ADC, A::IMM, 2),OpCode::new(/*0x6A*/ I::ROR, A::ACC, 2),OpCode::new(/*0x6B*/ I::XXX, A::IMP, 2),OpCode::new(/*0x6C*/ I::JMP, A::IND, 5),OpCode::new(/*0x6D*/ I::ADC, A::ABS, 4),OpCode::new(/*0x6E*/ I::ROR, A::ABS, 6),OpCode::new(/*0x6F*/ I::XXX, A::IMP, 6),
        OpCode::new(/*0x70*/ I::BVS, A::REL, 2),OpCode::new(/*0x71*/ I::ADC, A::IZY, 5),OpCode::new(/*0x72*/ I::XXX, A::IMP, 2),OpCode::new(/*0x73*/ I::XXX, A::IMP, 8),OpCode::new(/*0x74*/ I::NOP, A::IMP, 4),OpCode::new(/*0x75*/ I::ADC, A::ZPX, 4),OpCode::new(/*0x76*/ I::ROR, A::ZPX, 6),OpCode::new(/*0x77*/ I::XXX, A::IMP, 6),OpCode::new(/*0x78*/ I::SEI, A::IMP, 2),OpCode::new(/*0x79*/ I::ADC, A::ABY, 4),OpCode::new(/*0x7A*/ I::NOP, A::IMP, 2),OpCode::new(/*0x7B*/ I::XXX, A::IMP, 7),OpCode::new(/*0x7C*/ I::NOP, A::IMP, 4),OpCode::new(/*0x7D*/ I::ADC, A::ABX, 4),OpCode::new(/*0x7E*/ I::ROR, A::ABX, 7),OpCode::new(/*0x7F*/ I::XXX, A::IMP, 7),
        OpCode::new(/*0x80*/ I::NOP, A::IMP, 2),OpCode::new(/*0x81*/ I::STA, A::IZX, 6),OpCode::new(/*0x82*/ I::NOP, A::IMP, 2),OpCode::new(/*0x83*/ I::XXX, A::IMP, 6),OpCode::new(/*0x84*/ I::STY, A::ZP0, 3),OpCode::new(/*0x85*/ I::STA, A::ZP0, 3),OpCode::new(/*0x86*/ I::STX, A::ZP0, 3),OpCode::new(/*0x87*/ I::XXX, A::IMP, 3),OpCode::new(/*0x88*/ I::DEY, A::IMP, 2),OpCode::new(/*0x89*/ I::NOP, A::IMP, 2),OpCode::new(/*0x8A*/ I::TXA, A::IMP, 2),OpCode::new(/*0x8B*/ I::XXX, A::IMP, 2),OpCode::new(/*0x8C*/ I::STY, A::ABS, 4),OpCode::new(/*0x8D*/ I::STA, A::ABS, 4),OpCode::new(/*0x8E*/ I::STX, A::ABS, 4),OpCode::new(/*0x8F*/ I::XXX, A::IMP, 4),
        OpCode::new(/*0x90*/ I::BCC, A::REL, 2),OpCode::new(/*0x91*/ I::STA, A::IZY, 6),OpCode::new(/*0x92*/ I::XXX, A::IMP, 2),OpCode::new(/*0x93*/ I::XXX, A::IMP, 6),OpCode::new(/*0x94*/ I::STY, A::ZPX, 4),OpCode::new(/*0x95*/ I::STA, A::ZPX, 4),OpCode::new(/*0x96*/ I::STX, A::ZPY, 4),OpCode::new(/*0x97*/ I::XXX, A::IMP, 4),OpCode::new(/*0x98*/ I::TYA, A::IMP, 2),OpCode::new(/*0x99*/ I::STA, A::ABY, 5),OpCode::new(/*0x9A*/ I::TXS, A::IMP, 2),OpCode::new(/*0x9B*/ I::XXX, A::IMP, 5),OpCode::new(/*0x9C*/ I::NOP, A::IMP, 5),OpCode::new(/*0x9D*/ I::STA, A::ABX, 5),OpCode::new(/*0x9E*/ I::XXX, A::IMP, 5),OpCode::new(/*0x9F*/ I::XXX, A::IMP, 5),
        OpCode::new(/*0xA0*/ I::LDY, A::IMM, 2),OpCode::new(/*0xA1*/ I::LDA, A::IZX, 6),OpCode::new(/*0xA2*/ I::LDX, A::IMM, 2),OpCode::new(/*0xA3*/ I::XXX, A::IMP, 6),OpCode::new(/*0xA4*/ I::LDY, A::ZP0, 3),OpCode::new(/*0xA5*/ I::LDA, A::ZP0, 3),OpCode::new(/*0xA6*/ I::LDX, A::ZP0, 3),OpCode::new(/*0xA7*/ I::XXX, A::IMP, 3),OpCode::new(/*0xA8*/ I::TAY, A::IMP, 2),OpCode::new(/*0xA9*/ I::LDA, A::IMM, 2),OpCode::new(/*0xAA*/ I::TAX, A::IMP, 2),OpCode::new(/*0xAB*/ I::XXX, A::IMP, 2),OpCode::new(/*0xAC*/ I::LDY, A::ABS, 4),OpCode::new(/*0xAD*/ I::LDA, A::ABS, 4),OpCode::new(/*0xAE*/ I::LDX, A::ABS, 4),OpCode::new(/*0xAF*/ I::XXX, A::IMP, 4),
        OpCode::new(/*0xB0*/ I::BCS, A::REL, 2),OpCode::new(/*0xB1*/ I::LDA, A::IZY, 5),OpCode::new(/*0xB2*/ I::XXX, A::IMP, 2),OpCode::new(/*0xB3*/ I::XXX, A::IMP, 5),OpCode::new(/*0xB4*/ I::LDY, A::ZPX, 4),OpCode::new(/*0xB5*/ I::LDA, A::ZPX, 4),OpCode::new(/*0xB6*/ I::LDX, A::ZPY, 4),OpCode::new(/*0xB7*/ I::XXX, A::IMP, 4),OpCode::new(/*0xB8*/ I::CLV, A::IMP, 2),OpCode::new(/*0xB9*/ I::LDA, A::ABY, 4),OpCode::new(/*0xBA*/ I::TSX, A::IMP, 2),OpCode::new(/*0xBB*/ I::XXX, A::IMP, 4),OpCode::new(/*0xBC*/ I::LDY, A::ABX, 4),OpCode::new(/*0xBD*/ I::LDA, A::ABX, 4),OpCode::new(/*0xBE*/ I::LDX, A::ABY, 4),OpCode::new(/*0xBF*/ I::XXX, A::IMP, 4),
        OpCode::new(/*0xC0*/ I::CPY, A::IMM, 2),OpCode::new(/*0xC1*/ I::CMP, A::IZX, 6),OpCode::new(/*0xC2*/ I::NOP, A::IMP, 2),OpCode::new(/*0xC3*/ I::XXX, A::IMP, 8),OpCode::new(/*0xC4*/ I::CPY, A::ZP0, 3),OpCode::new(/*0xC5*/ I::CMP, A::ZP0, 3),OpCode::new(/*0xC6*/ I::DEC, A::ZP0, 5),OpCode::new(/*0xC7*/ I::XXX, A::IMP, 5),OpCode::new(/*0xC8*/ I::INY, A::IMP, 2),OpCode::new(/*0xC9*/ I::CMP, A::IMM, 2),OpCode::new(/*0xCA*/ I::DEX, A::IMP, 2),OpCode::new(/*0xCB*/ I::XXX, A::IMP, 2),OpCode::new(/*0xCC*/ I::CPY, A::ABS, 4),OpCode::new(/*0xCD*/ I::CMP, A::ABS, 4),OpCode::new(/*0xCE*/ I::DEC, A::ABS, 6),OpCode::new(/*0xCF*/ I::XXX, A::IMP, 6),
        OpCode::new(/*0xD0*/ I::BNE, A::REL, 2),OpCode::new(/*0xD1*/ I::CMP, A::IZY, 5),OpCode::new(/*0xD2*/ I::XXX, A::IMP, 2),OpCode::new(/*0xD3*/ I::XXX, A::IMP, 8),OpCode::new(/*0xD4*/ I::NOP, A::IMP, 4),OpCode::new(/*0xD5*/ I::CMP, A::ZPX, 4),OpCode::new(/*0xD6*/ I::DEC, A::ZPX, 6),OpCode::new(/*0xD7*/ I::XXX, A::IMP, 6),OpCode::new(/*0xD8*/ I::CLD, A::IMP, 2),OpCode::new(/*0xD9*/ I::CMP, A::ABY, 4),OpCode::new(/*0xDA*/ I::NOP, A::IMP, 2),OpCode::new(/*0xDB*/ I::XXX, A::IMP, 7),OpCode::new(/*0xDC*/ I::NOP, A::IMP, 4),OpCode::new(/*0xDD*/ I::CMP, A::ABX, 4),OpCode::new(/*0xDE*/ I::DEC, A::ABX, 7),OpCode::new(/*0xDF*/ I::XXX, A::IMP, 7),
        OpCode::new(/*0xE0*/ I::CPX, A::IMM, 2),OpCode::new(/*0xE1*/ I::SBC, A::IZX, 6),OpCode::new(/*0xE2*/ I::NOP, A::IMP, 2),OpCode::new(/*0xE3*/ I::XXX, A::IMP, 8),OpCode::new(/*0xE4*/ I::CPX, A::ZP0, 3),OpCode::new(/*0xE5*/ I::SBC, A::ZP0, 3),OpCode::new(/*0xE6*/ I::INC, A::ZP0, 5),OpCode::new(/*0xE7*/ I::XXX, A::IMP, 5),OpCode::new(/*0xE8*/ I::INX, A::IMP, 2),OpCode::new(/*0xE9*/ I::SBC, A::IMM, 2),OpCode::new(/*0xEA*/ I::NOP, A::IMP, 2),OpCode::new(/*0xEB*/ I::SBC, A::IMP, 2),OpCode::new(/*0xEC*/ I::CPX, A::ABS, 4),OpCode::new(/*0xED*/ I::SBC, A::ABS, 4),OpCode::new(/*0xEE*/ I::INC, A::ABS, 6),OpCode::new(/*0xEF*/ I::XXX, A::IMP, 6),
        OpCode::new(/*0xF0*/ I::BEQ, A::REL, 2),OpCode::new(/*0xF1*/ I::SBC, A::IZY, 5),OpCode::new(/*0xF2*/ I::XXX, A::IMP, 2),OpCode::new(/*0xF3*/ I::XXX, A::IMP, 8),OpCode::new(/*0xF4*/ I::NOP, A::IMP, 4),OpCode::new(/*0xF5*/ I::SBC, A::ZPX, 4),OpCode::new(/*0xF6*/ I::INC, A::ZPX, 6),OpCode::new(/*0xF7*/ I::XXX, A::IMP, 6),OpCode::new(/*0xF8*/ I::SED, A::IMP, 2),OpCode::new(/*0xF9*/ I::SBC, A::ABY, 4),OpCode::new(/*0xFA*/ I::NOP, A::IMP, 2),OpCode::new(/*0xFB*/ I::XXX, A::IMP, 7),OpCode::new(/*0xFC*/ I::NOP, A::IMP, 4),OpCode::new(/*0xFD*/ I::SBC, A::ABX, 4),OpCode::new(/*0xFE*/ I::INC, A::ABX, 7),OpCode::new(/*0xFF*/ I::XXX, A::IMP, 7),
    ];
}
