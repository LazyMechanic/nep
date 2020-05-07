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
