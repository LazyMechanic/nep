use super::color::Color;
use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Pixel {
    pub color: Color,
    pub x:     usize,
    pub y:     usize,
}

impl Pixel {
    pub fn new(color: Color, x: usize, y: usize) -> Self {
        Self { color, x, y }
    }
}
