use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy)]
pub struct PpuScroll {
    x:    Byte,
    y:    Byte,
    axis: Axis,
}

impl PpuScroll {
    pub fn new() -> Self {
        Self {
            x:    0x00.into(),
            y:    0x00.into(),
            axis: Axis::X,
        }
    }

    pub fn set_cur_axis(&mut self, axis: Axis) {
        self.axis = axis;
    }

    pub fn set_cur_axis_x(&mut self) {
        self.set_cur_axis(Axis::X);
    }

    pub fn set_cur_axis_y(&mut self) {
        self.set_cur_axis(Axis::Y);
    }

    pub fn x(&self) -> Byte {
        self.x
    }

    pub fn y(&self) -> Byte {
        self.y
    }

    pub fn write(&mut self, v: Byte) {
        match self.axis {
            Axis::X => {
                self.x = v;
                self.axis = Axis::Y;
            }
            Axis::Y => {
                self.y = v;
                self.axis = Axis::X;
            }
        }
    }
}
