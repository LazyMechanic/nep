pub struct StatusRegister {
    pub carry:             bool, // 00000001
    pub zero:              bool, // 00000010
    pub disable_interrupt: bool, // 00000100
    pub decimal_mode:      bool, // 00001000
    pub break_mode:        bool, // 00010000
    pub reserved:          bool, // 00100000
    pub overflow:          bool, // 01000000
    pub negative:          bool, // 10000000
}

impl StatusRegister {
    pub fn new() -> StatusRegister {
        StatusRegister {
            carry:             false,
            zero:              false,
            disable_interrupt: true,
            decimal_mode:      false,
            break_mode:        true,
            reserved:          true,
            overflow:          false,
            negative:          false,
        }
    }
}

impl From<u8> for StatusRegister {
    fn from(v: u8) -> Self {
        StatusRegister {
            carry:             v & 0b00000001 == 0b00000001,
            zero:              v & 0b00000010 == 0b00000010,
            disable_interrupt: v & 0b00000100 == 0b00000100,
            decimal_mode:      v & 0b00001000 == 0b00001000,
            break_mode:        v & 0b00010000 == 0b00010000,
            reserved:          v & 0b00100000 == 0b00100000,
            overflow:          v & 0b01000000 == 0b01000000,
            negative:          v & 0b10000000 == 0b10000000,
        }
    }
}
