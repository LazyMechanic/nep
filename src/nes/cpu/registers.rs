use super::status_register::StatusRegister;
use crate::prelude::*;

pub trait CpuRegisters {
    fn get_a(&self) -> Byte;
    fn get_x(&self) -> Byte;
    fn get_y(&self) -> Byte;
    fn get_sp(&self) -> Byte;
    fn get_pc(&self) -> Addr;
    fn get_carry(&self) -> bool;
    fn get_zero(&self) -> bool;
    fn get_interrupt(&self) -> bool;
    fn get_decimal_mode(&self) -> bool;
    fn get_break_mode(&self) -> bool;
    fn get_reserved(&self) -> bool;
    fn get_overflow(&self) -> bool;
    fn get_negative(&self) -> bool;
    fn get_status(&self) -> Byte;

    fn set_a(&mut self, v: Byte) -> &mut Self;
    fn set_x(&mut self, v: Byte) -> &mut Self;
    fn set_y(&mut self, v: Byte) -> &mut Self;
    fn set_sp(&mut self, v: Byte) -> &mut Self;
    fn set_pc(&mut self, v: Addr) -> &mut Self;
    fn set_carry(&mut self, v: bool) -> &mut Self;
    fn set_zero(&mut self, v: bool) -> &mut Self;
    fn set_interrupt(&mut self, v: bool) -> &mut Self;
    fn set_decimal_mode(&mut self, v: bool) -> &mut Self;
    fn set_break_mode(&mut self, v: bool) -> &mut Self;
    fn set_reserved(&mut self, v: bool) -> &mut Self;
    fn set_overflow(&mut self, v: bool) -> &mut Self;
    fn set_negative(&mut self, v: bool) -> &mut Self;
    fn set_status(&mut self, v: Byte) -> &mut Self;

    fn inc_sp(&mut self) -> &mut Self;
    fn inc_pc(&mut self) -> &mut Self;
    fn dec_sp(&mut self) -> &mut Self;
    fn dec_pc(&mut self) -> &mut Self;
    fn update_negative_by(&mut self, v: Byte) -> &mut Self;
    fn update_zero_by(&mut self, v: Byte) -> &mut Self;
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Registers {
    pub a:      Byte,
    // Accumulator register
    pub x:      Byte,
    // X register
    pub y:      Byte,
    // Y register
    pub sp:     Byte,
    // Stack pointer
    pub pc:     Addr,
    // Program counter
    pub status: StatusRegister, // Status register
}

impl CpuRegisters for Registers {
    fn get_a(&self) -> Byte {
        self.a
    }

    fn get_x(&self) -> Byte {
        self.x
    }

    fn get_y(&self) -> Byte {
        self.y
    }

    fn get_sp(&self) -> Byte {
        self.sp
    }

    fn get_pc(&self) -> Addr {
        self.pc
    }

    fn get_carry(&self) -> bool {
        self.status.carry
    }

    fn get_zero(&self) -> bool {
        self.status.zero
    }

    fn get_interrupt(&self) -> bool {
        self.status.interrupt
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

    fn get_status(&self) -> Byte {
        Byte(self.status.clone().into())
    }

    fn set_a(&mut self, v: Byte) -> &mut Self {
        self.a = v;
        self
    }

    fn set_x(&mut self, v: Byte) -> &mut Self {
        self.x = v;
        self
    }

    fn set_y(&mut self, v: Byte) -> &mut Self {
        self.y = v;
        self
    }

    fn set_sp(&mut self, v: Byte) -> &mut Self {
        self.sp = v;
        self
    }

    fn set_pc(&mut self, v: Addr) -> &mut Self {
        self.pc = v;
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

    fn set_interrupt(&mut self, v: bool) -> &mut Self {
        self.status.interrupt = v;
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

    fn set_status(&mut self, v: Byte) -> &mut Self {
        self.status = v.into();
        self
    }

    fn inc_sp(&mut self) -> &mut Self {
        self.sp.inc();
        self
    }

    fn inc_pc(&mut self) -> &mut Self {
        self.pc.inc();
        self
    }

    fn dec_sp(&mut self) -> &mut Self {
        self.sp.dec();
        self
    }

    fn dec_pc(&mut self) -> &mut Self {
        self.pc.dec();
        self
    }

    fn update_negative_by(&mut self, v: Byte) -> &mut Self {
        self.status.negative = v.is_neg();
        self
    }

    fn update_zero_by(&mut self, v: Byte) -> &mut Self {
        self.status.zero = v.is_clear();
        self
    }
}
