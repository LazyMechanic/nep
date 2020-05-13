use std::time;

const CPU_COUNT: u64 = 3;
const PPU_COUNT: u64 = 1;

#[derive(Debug, Default)]
pub struct Clock {
    counter: u64,
}

impl Clock {
    pub fn reset(&mut self) {
        self.counter = 0;
    }

    pub fn update(&mut self) {
        self.counter = self.counter.overflowing_add(1).0;
    }

    pub fn need_step_cpu(&self) -> bool {
        self.counter % CPU_COUNT == 0
    }

    pub fn need_step_ppu(&self) -> bool {
        self.counter % PPU_COUNT == 0
    }

    pub fn counter(&self) -> u64 {
        self.counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update() {
        let mut clock = Clock::default();
        let mut cpu = 0u8;
        let mut ppu = 0u8;

        for i in 0..24 {
            clock.update();
            if clock.need_step_cpu() {
                cpu += 1;
            }
            if clock.need_step_ppu() {
                ppu += 1;
            }
        }

        assert_eq!(cpu, 8, "cpu counter not equal");
        assert_eq!(ppu, 24, "ppu counter not equal");
    }
}
