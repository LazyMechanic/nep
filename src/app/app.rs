use std::env;

use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use super::consts;

pub struct App {
    sdl_context: Sdl,
    canvas:      WindowCanvas,
}

impl App {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(consts::WINDOW_TITLE, consts::WIDTH, consts::HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        App {
            sdl_context,
            canvas,
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) -> nep::Result<()> { Ok(()) }

    pub fn run(&mut self) { loop {} }
}
