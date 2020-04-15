use super::status_register::StatusRegister;

pub struct Cpu {}

impl Cpu {
    pub fn new() -> Self {
        Cpu {}
    }

    pub fn reset(&mut self) {}

    pub fn irq(&mut self) {}

    pub fn nmi(&mut self) {}

    pub fn update(&mut self) {}
}
