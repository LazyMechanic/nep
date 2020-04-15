use super::status_register::StatusRegister;

pub trait CpuRegisters {
    fn get_a(&self) -> u8;
    fn get_x(&self) -> u8;
    fn get_y(&self) -> u8;
    fn get_sp(&self) -> u8;
    fn get_pc(&self) -> u16;
    fn get_status(&self) -> u8;
    fn get_carry(&self) -> bool;
    fn get_zero(&self) -> bool;
    fn get_disable_interrupt(&self) -> bool;
    fn get_decimal_mode(&self) -> bool;
    fn get_break_mode(&self) -> bool;
    fn get_reserved(&self) -> bool;
    fn get_overflow(&self) -> bool;
    fn get_negative(&self) -> bool;

    fn set_a(&mut self, v: u8) -> &mut Self;
    fn set_x(&mut self, v: u8) -> &mut Self;
    fn set_y(&mut self, v: u8) -> &mut Self;
    fn set_sp(&mut self, v: u8) -> &mut Self;
    fn set_pc(&mut self, v: u16) -> &mut Self;
    fn set_status(&mut self, v: u8) -> &mut Self;
    fn set_carry(&mut self, v: bool) -> &mut Self;
    fn set_zero(&mut self, v: bool) -> &mut Self;
    fn set_disable_interrupt(&mut self, v: bool) -> &mut Self;
    fn set_decimal_mode(&mut self, v: bool) -> &mut Self;
    fn set_break_mode(&mut self, v: bool) -> &mut Self;
    fn set_reserved(&mut self, v: bool) -> &mut Self;
    fn set_overflow(&mut self, v: bool) -> &mut Self;
    fn set_negative(&mut self, v: bool) -> &mut Self;

    fn inc_sp(&mut self) -> &mut Self;
    fn inc_pc(&mut self) -> &mut Self;

    fn dec_sp(&mut self) -> &mut Self;
    fn dec_pc(&mut self) -> &mut Self;

    fn update_negative_by(&mut self, v: u8) -> &mut Self;
    fn update_zero_by(&mut self, v: u8) -> &mut Self;
}

pub struct Registers {
    pub a:      u8,             // Accumulator register
    pub x:      u8,             // X register
    pub y:      u8,             // Y register
    pub sp:     u8,             // Stack pointer
    pub pc:     u16,            // Program counter
    pub status: StatusRegister, // Status register
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a:      0,
            x:      0,
            y:      0,
            sp:     0x8000,
            pc:     0xfd,
            status: StatusRegister::new(),
        }
    }
}

impl CpuRegisters for Registers {
    fn get_a(&self) -> u8 {
        self.a
    }

    fn get_x(&self) -> u8 {
        self.x
    }

    fn get_y(&self) -> u8 {
        self.y
    }

    fn get_sp(&self) -> u8 {
        self.sp
    }

    fn get_pc(&self) -> u16 {
        self.pc
    }

    fn get_status(&self) -> u8 {
        self.status.into()
    }

    fn get_carry(&self) -> bool {
        self.status.carry
    }

    fn get_zero(&self) -> bool {
        self.status.zero
    }

    fn get_disable_interrupt(&self) -> bool {
        self.status.disable_interrupt
    }

    fn get_decimal_mode(&self) -> bool {
        self.status.decimal_mode
    }

    fn get_break_mode(&self) -> bool {
        self.status.break_mode
    }

    fn get_reserved(&self) -> bool {
        self.status.reserved
    }

    fn get_overflow(&self) -> bool {
        self.status.overflow
    }

    fn get_negative(&self) -> bool {
        self.status.negative
    }

    fn set_a(&mut self, v: u8) -> &mut Self {
        self.a = v;
        self
    }

    fn set_x(&mut self, v: u8) -> &mut Self {
        self.x = v;
        self
    }

    fn set_y(&mut self, v: u8) -> &mut Self {
        self.y = v;
        self
    }

    fn set_sp(&mut self, v: u8) -> &mut Self {
        self.sp = v;
        self
    }

    fn set_pc(&mut self, v: u16) -> &mut Self {
        self.pc = v;
        self
    }

    fn set_status(&mut self, v: u8) -> &mut Self {
        self.status = StatusRegister::from(v);
        self
    }

    fn set_carry(&mut self, v: bool) -> &mut Self {
        self.status.carry = v;
        self
    }

    fn set_zero(&mut self, v: bool) -> &mut Self {
        self.status.zero = v;
        self
    }

    fn set_disable_interrupt(&mut self, v: bool) -> &mut Self {
        self.status.disable_interrupt = v;
        self
    }

    fn set_decimal_mode(&mut self, v: bool) -> &mut Self {
        self.status.decimal_mode = v;
        self
    }

    fn set_break_mode(&mut self, v: bool) -> &mut Self {
        self.status.break_mode = v;
        self
    }

    fn set_reserved(&mut self, v: bool) -> &mut Self {
        self.status.reserved = v;
        self
    }

    fn set_overflow(&mut self, v: bool) -> &mut Self {
        self.status.overflow = v;
        self
    }

    fn set_negative(&mut self, v: bool) -> &mut Self {
        self.status.negative = v;
        self
    }

    fn inc_sp(&mut self) -> &mut Self {
        self.sp += 1;
        self
    }

    fn inc_pc(&mut self) -> &mut Self {
        self.pc += 1;
        self
    }

    fn dec_sp(&mut self) -> &mut Self {
        self.sp -= 1;
        self
    }

    fn dec_pc(&mut self) -> &mut Self {
        self.pc -= 1;
        self
    }

    fn update_negative_by(&mut self, v: u8) -> &mut Self {
        self.status.negative = v & 0x80 == 0x80;
        self
    }

    fn update_zero_by(&mut self, v: u8) -> &mut Self {
        self.status.zero = v == 0;
        self
    }
}
