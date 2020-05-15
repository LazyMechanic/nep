use super::bus::CpuBus;
use super::status_register::StatusRegister;
use crate::prelude::*;

pub trait CpuRegisters {
    fn a(&self) -> Byte;
    fn x(&self) -> Byte;
    fn y(&self) -> Byte;
    fn sp(&self) -> Byte;
    fn pc(&self) -> Addr;
    fn carry(&self) -> bool;
    fn zero(&self) -> bool;
    fn interrupt(&self) -> bool;
    fn decimal_mode(&self) -> bool;
    fn break_mode(&self) -> bool;
    fn reserved(&self) -> bool;
    fn overflow(&self) -> bool;
    fn negative(&self) -> bool;
    fn status(&self) -> Byte;

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

impl Registers {
    pub fn reset<T>(&mut self, bus: &mut T)
    where
        T: CpuBus,
    {
        let a: Byte = 0x00.into();
        let x: Byte = 0x00.into();
        let y: Byte = 0x00.into();
        let sp: Byte = 0xFD.into();
        let status = StatusRegister::new();
        let pc = {
            let mut pc_addr_base = 0xFFFC.into();
            let lo = bus.read(pc_addr_base);
            let hi = bus.read(pc_addr_base.inc());
            Addr::from_bytes(lo, hi)
        };

        self.a = a;
        self.x = x;
        self.y = y;
        self.sp = sp;
        self.status = status;
        self.pc = pc;
    }
}

impl CpuRegisters for Registers {
    fn a(&self) -> Byte {
        self.a
    }

    fn x(&self) -> Byte {
        self.x
    }

    fn y(&self) -> Byte {
        self.y
    }

    fn sp(&self) -> Byte {
        self.sp
    }

    fn pc(&self) -> Addr {
        self.pc
    }

    fn carry(&self) -> bool {
        self.status.carry()
    }

    fn zero(&self) -> bool {
        self.status.zero()
    }

    fn interrupt(&self) -> bool {
        self.status.interrupt()
    }

    fn decimal_mode(&self) -> bool {
        self.status.decimal_mode()
    }

    fn break_mode(&self) -> bool {
        self.status.break_mode()
    }

    fn reserved(&self) -> bool {
        self.status.reserved()
    }

    fn overflow(&self) -> bool {
        self.status.overflow()
    }

    fn negative(&self) -> bool {
        self.status.negative()
    }

    fn status(&self) -> Byte {
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
        self.status.set_carry(v);
        self
    }

    fn set_zero(&mut self, v: bool) -> &mut Self {
        self.status.set_zero(v);
        self
    }

    fn set_interrupt(&mut self, v: bool) -> &mut Self {
        self.status.set_interrupt(v);
        self
    }

    fn set_decimal_mode(&mut self, v: bool) -> &mut Self {
        self.status.set_decimal_mode(v);
        self
    }

    fn set_break_mode(&mut self, v: bool) -> &mut Self {
        self.status.set_break_mode(v);
        self
    }

    fn set_reserved(&mut self, v: bool) -> &mut Self {
        self.status.set_reserved(v);
        self
    }

    fn set_overflow(&mut self, v: bool) -> &mut Self {
        self.status.set_overflow(v);
        self
    }

    fn set_negative(&mut self, v: bool) -> &mut Self {
        self.status.set_negative(v);
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
        self.set_negative(v.is_neg())
    }

    fn update_zero_by(&mut self, v: Byte) -> &mut Self {
        self.set_zero(v.is_clear())
    }
}
