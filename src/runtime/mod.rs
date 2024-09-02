
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;
#[cfg(target_arch = "wasm32")]
use web_sys::js_sys::Promise;

use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::Mutex;
use std::time::{self, Duration};
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::hardware::Emulator;
use crate::hardware::Variant;
use crate::graphics::{self, State};
use crate::hardware::SCREEN_HEIGHT;
use crate::hardware::SCREEN_WIDTH;


/*/
use lazy_static::lazy_static;

lazy_static! {
    static ref MY_VEC: Mutex<Vec<u8>> = Mutex::new(Vec::new());
}*/

use crate::util::sleep;

const CPU_CYCLE_PERIOD: time::Duration = time::Duration::from_millis(4);

const TIMER_CYCLE_PERIOD: time::Duration = time::Duration::from_millis(20);
pub struct Runtime<'a> {
    chip8: Emulator,
    timeref: instant::Instant,
    timerefclock: instant::Instant,
    pub graphics: State<'a>,
}

impl<'a> Runtime<'a> {
    pub async fn new(graphics: State<'a>,data: js_sys::Uint8Array) -> Self {
        let mut chip8 = Emulator::new(Variant::Legacy);
        chip8.load_characters();

        /*
        let name = "IBM Logo.ch8";
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
            let name = "http://localhost:3000/roms/".to_owned() + name;
            chip8.fetch_and_load_rom(&name).await;
            } else {
                let name = "roms/".to_owned() + name;
                chip8.load_rom(&name);
            }
        }*/
        sleep(250).await;
        chip8.load_rom_from_vec(data);




        let timeref = instant::Instant::now();
        let timerefclock = instant::Instant::now();
        Self { chip8, timeref, timerefclock, graphics }
    }

    pub fn input(&mut self, key: &winit::event::KeyEvent) {
        let value = key.state == ElementState::Pressed;

        match key.physical_key {
            PhysicalKey::Code(KeyCode::Digit1) => self.chip8.set_key(0x1, value),
            PhysicalKey::Code(KeyCode::Digit2) => self.chip8.set_key(0x2, value),
            PhysicalKey::Code(KeyCode::Digit3) => self.chip8.set_key(0x3, value),
            PhysicalKey::Code(KeyCode::Digit4) => self.chip8.set_key(0xC, value),
            PhysicalKey::Code(KeyCode::KeyQ) => self.chip8.set_key(0x4, value),
            PhysicalKey::Code(KeyCode::KeyW) => self.chip8.set_key(0x5, value),
            PhysicalKey::Code(KeyCode::KeyE) => self.chip8.set_key(0x6, value),
            PhysicalKey::Code(KeyCode::KeyR) => self.chip8.set_key(0xD, value),
            PhysicalKey::Code(KeyCode::KeyA) => self.chip8.set_key(0x7, value),
            PhysicalKey::Code(KeyCode::KeyS) => self.chip8.set_key(0x8, value),
            PhysicalKey::Code(KeyCode::KeyD) => self.chip8.set_key(0x9, value),
            PhysicalKey::Code(KeyCode::KeyF) => self.chip8.set_key(0xE, value),
            PhysicalKey::Code(KeyCode::KeyZ) => self.chip8.set_key(0xA, value),
            PhysicalKey::Code(KeyCode::KeyX) => self.chip8.set_key(0x0, value),
            PhysicalKey::Code(KeyCode::KeyC) => self.chip8.set_key(0xB, value),
            PhysicalKey::Code(KeyCode::KeyV) => self.chip8.set_key(0xF, value),
            _ => {}
        }
    }

    pub fn run(&mut self) {
        

        if self.timerefclock.elapsed() >= TIMER_CYCLE_PERIOD {
            self.chip8.timer_cycle();
            self.timerefclock = instant::Instant::now();
        }

        if self.chip8.screen_changed{
            self.graphics.update_buffer(&self.chip8.getscreen());
            let _ = self.graphics.call_fast_render();
            //log::warn!("USER::: Screen Changed");
            self.chip8.screen_changed = false;
        }



        if self.timeref.elapsed() >= CPU_CYCLE_PERIOD {
            self.chip8.cpu_cycle();
            self.timeref = instant::Instant::now();
        }

        

        
      
        

    }
}