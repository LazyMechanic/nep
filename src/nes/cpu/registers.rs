use super::bus::Bus;
use super::status_register::StatusRegister;
use crate::prelude::*;

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

impl Registers {
    pub fn reset(&mut self, cpu_bus: &mut Bus) {
        let a: Byte = 0x00.into();
        let x: Byte = 0x00.into();
        let y: Byte = 0x00.into();
        let sp: Byte = 0xFD.into();
        let status = StatusRegister::new();
        let pc = {
            let mut pc_addr_base = 0xFFFC.into();
            let lo = cpu_bus.read(pc_addr_base);
            let hi = cpu_bus.read(pc_addr_base.inc());
            Addr::from_bytes(lo, hi)
        };

        self.a = a;
        self.x = x;
        self.y = y;
        self.sp = sp;
        self.status = status;
        self.pc = pc;
    }

    pub fn a(&self) -> Byte {
        self.a
    }

    pub fn x(&self) -> Byte {
        self.x
    }

    pub fn y(&self) -> Byte {
        self.y
    }

    pub fn sp(&self) -> Byte {
        self.sp
    }

    pub fn pc(&self) -> Addr {
        self.pc
    }

    pub fn carry(&self) -> bool {
        self.status.carry()
    }

    pub fn zero(&self) -> bool {
        self.status.zero()
    }

    pub fn interrupt(&self) -> bool {
        self.status.interrupt()
    }

    pub fn decimal_mode(&self) -> bool {
        self.status.decimal_mode()
    }

    pub fn break_mode(&self) -> bool {
        self.status.break_mode()
    }

    pub fn reserved(&self) -> bool {
        self.status.reserved()
    }

    pub fn overflow(&self) -> bool {
        self.status.overflow()
    }

    pub fn negative(&self) -> bool {
        self.status.negative()
    }

    pub fn status(&self) -> Byte {
        Byte(self.status.clone().into())
    }

    pub fn set_a(&mut self, v: Byte) -> &mut Self {
        self.a = v;
        self
    }

    pub fn set_x(&mut self, v: Byte) -> &mut Self {
        self.x = v;
        self
    }

    pub fn set_y(&mut self, v: Byte) -> &mut Self {
        self.y = v;
        self
    }

    pub fn set_sp(&mut self, v: Byte) -> &mut Self {
        self.sp = v;
        self
    }

    pub fn set_pc(&mut self, v: Addr) -> &mut Self {
        self.pc = v;
        self
    }

    pub fn set_carry(&mut self, v: bool) -> &mut Self {
        self.status.set_carry(v);
        self
    }

    pub fn set_zero(&mut self, v: bool) -> &mut Self {
        self.status.set_zero(v);
        self
    }

    pub fn set_interrupt(&mut self, v: bool) -> &mut Self {
        self.status.set_interrupt(v);
        self
    }

    pub fn set_decimal_mode(&mut self, v: bool) -> &mut Self {
        self.status.set_decimal_mode(v);
        self
    }

    pub fn set_break_mode(&mut self, v: bool) -> &mut Self {
        self.status.set_break_mode(v);
        self
    }

    pub fn set_reserved(&mut self, v: bool) -> &mut Self {
        self.status.set_reserved(v);
        self
    }

    pub fn set_overflow(&mut self, v: bool) -> &mut Self {
        self.status.set_overflow(v);
        self
    }

    pub fn set_negative(&mut self, v: bool) -> &mut Self {
        self.status.set_negative(v);
        self
    }

    pub fn set_status(&mut self, v: Byte) -> &mut Self {
        self.status = v.into();
        self
    }

    pub fn inc_sp(&mut self) -> &mut Self {
        self.sp.inc();
        self
    }

    pub fn inc_pc(&mut self) -> &mut Self {
        self.pc.inc();
        self
    }

    pub fn dec_sp(&mut self) -> &mut Self {
        self.sp.dec();
        self
    }

    pub fn dec_pc(&mut self) -> &mut Self {
        self.pc.dec();
        self
    }

    pub fn update_negative_by(&mut self, v: Byte) -> &mut Self {
        self.set_negative(v.is_neg())
    }

    pub fn update_zero_by(&mut self, v: Byte) -> &mut Self {
        self.set_zero(v.is_clear())
    }
}
