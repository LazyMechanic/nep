use std::env;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::SwapInterval;
use sdl2::Sdl;

use nep::prelude::*;
use nep::Emu;

use super::consts;

use std::convert::TryInto;
use std::io::{Read, Seek};
use std::path::Path;

pub struct App {
    sdl_context: Sdl,
    canvas:      WindowCanvas,
    emu:         Emu,
}

fn keycode_to_pad(key: Keycode) -> u8 {
    match key {
        Keycode::O => consts::PAD_A,
        Keycode::P => consts::PAD_B,
        Keycode::K => consts::PAD_SELECT,
        Keycode::L => consts::PAD_START,
        Keycode::W => consts::PAD_U,
        Keycode::S => consts::PAD_D,
        Keycode::A => consts::PAD_L,
        Keycode::D => consts::PAD_R,
        _ => 0,
    }
}

impl App {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        video_subsystem.gl_set_swap_interval(SwapInterval::Immediate);
        let window = video_subsystem
            .window(consts::WINDOW_TITLE, consts::WIDTH, consts::HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        App {
            sdl_context,
            canvas,
            emu: Emu::new(),
        }
    }

    pub fn load<F: Read + Seek>(&mut self, file: &mut F) -> Result<()> {
        self.emu.load(file)?;
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<()> {
        self.emu.load_from_file(file_path)?;
        Ok(())
    }

    pub fn render(&mut self) {
        self.canvas.clear();

        let screen = self.emu.screen();
        //if screen.ready {
        for p in screen.pixels().iter() {
            let color = Color::RGB(p.color.r.into(), p.color.g.into(), p.color.b.into());
            let point = Point::new(p.x as i32, p.y as i32);
            //println!("{:?}", color);
            self.canvas.set_draw_color(color);
            self.canvas.draw_point(point);
        }
        //}

        self.canvas.present();
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut joy_1_state = 0u8;
        let mut joy_2_state = 0u8;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        joy_1_state |= keycode_to_pad(key);
                    }
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        joy_1_state &= !keycode_to_pad(key);
                    }
                    _ => {}
                }
            }

            self.emu.update_joypads(joy_1_state, joy_2_state);
            self.emu.step();
            self.render();
        }
    }
}
