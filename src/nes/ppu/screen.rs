use super::pixel::Pixel;
use crate::prelude::*;

pub struct Screen {
    mem:       Vec<Pixel>,
    width:     usize,
    height:    usize,
    pub ready: bool,
}

impl Screen {
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            mem: vec![Pixel::default(); width * height],
            width,
            height,
            ready: false,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &Vec<Pixel> {
        &self.mem
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        if x < self.width && y < self.height {
            self.mem[y * self.width + x] = pixel;
        }
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Pixel {
        if x < self.width && y < self.height {
            self.mem[y * self.width + x]
        } else {
            Pixel::default()
        }
    }
}
