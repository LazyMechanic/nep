use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use nep::nes::context::Context;

const WINDOW_TITLE: &str = "nep";
const WIDTH: u32 = 256;
const HEIGHT: u32 = 224;

const PAD_A: u8 = 0x01;
const PAD_B: u8 = 0x02;
const PAD_SELECT: u8 = 0x04;
const PAD_START: u8 = 0x08;
const PAD_U: u8 = 0x10;
const PAD_D: u8 = 0x20;
const PAD_L: u8 = 0x40;
const PAD_R: u8 = 0x80;

pub struct App {
    sdl_context: Sdl,
    canvas: WindowCanvas,
    ctx: Option<Context>,
}

impl App {
    pub fn new() -> App {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(WINDOW_TITLE, WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        App {
            sdl_context,
            canvas,
            ctx: None,
        }
    }

    pub fn run(&mut self) {
        loop {}
    }
}
