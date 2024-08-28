

use std::sync::mpsc::SyncSender;

use std::{fs::File, io::Read};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(PartialEq)]
pub enum Variant {
    Legacy,
    Modern,
}

/*
// Play a tone at the specified frequency, volume, and duration
fn play_tone(frequency: u32, volume: f32, duration: Duration) {
    thread::spawn(move || {
        // Initialize the audio output stream
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Create a sine wave source at the specified frequency, amplify it, and limit its duration
        let source = SineWave::new(frequency)
            .amplify(volume)
            .take_duration(duration);

        // Play the sine wave for the specified duration
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    });
}*/

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const START_ADDRESS: u16 = 0x200;
const FONTSET_START_ADDRESS: usize = 0x50;
const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

pub struct Emulator {
    registers: [u8; NUM_REGISTERS],
    indexregister: u16,
    programcounter: u16,
    memory: [u8; RAM_SIZE],
    stack: [u16; STACK_SIZE],
    stackpointer: u8,
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub screen_changed: bool,
    keys: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
    variant: Variant,
    //display_sender: SyncSender<[bool; SCREEN_WIDTH * SCREEN_HEIGHT]>,
}

impl Emulator {
    pub fn getscreen(&self) -> [bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        self.screen
    }
    pub fn new(
        variant: Variant,
        //display_sender: SyncSender<[bool; SCREEN_WIDTH * SCREEN_HEIGHT]>,
    ) -> Self {
        Emulator {
            registers: [0; NUM_REGISTERS],
            indexregister: 0,
            programcounter: START_ADDRESS, //512 or 0x200
            memory: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            stackpointer: 0,
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            screen_changed: true,
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
            variant,
            //display_sender,
        }
    }

    pub fn load_rom(&mut self, filename: &str) {
        let mut file = File::open(filename).expect("Failed to open ROM file");
        let mut buffer = Vec::new();
        let mut end = file
            .read_to_end(&mut buffer)
            .expect("Failed to read ROM file");

        // Load the ROM into memory starting at 0x200
        let start = START_ADDRESS as usize;
        end += start;
        self.memory[start..end].copy_from_slice(&buffer);

        // Set the program counter to the start address
        self.programcounter = START_ADDRESS;
    }
    
    
    #[cfg(target_arch = "wasm32")]
    pub async fn fetch_and_load_rom(&mut self, url: &str) {
        use web_sys::js_sys::Uint8Array; // Add this line to import the Uint8Array type and JsValue
        
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::JsCast; // Add this line to import the JsCast trait
        
        let response = JsFuture::from(web_sys::window()
            .unwrap()
            .fetch_with_str(url))
            .await.unwrap();
        assert!(response.is_instance_of::<Response>());
        let resp: Response = response.clone().dyn_into().unwrap();
        let array_buffer = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
        let rom_data = Uint8Array::new(&array_buffer).to_vec();
        
        // Load the ROM into memory starting at 0x200
        let start = START_ADDRESS as usize;
        let end = start + rom_data.len();
        self.memory[start..end].copy_from_slice(&rom_data);
    
        // Set the program counter to the start address
        self.programcounter = START_ADDRESS;
    }
    
    pub fn load_characters(&mut self) {
        self.memory[..FONTSET_START_ADDRESS].copy_from_slice(&FONTSET);
    }

    fn push(&mut self, value: u16) {
        self.stack[self.stackpointer as usize] = value;
        self.stackpointer += 1;
    }

    fn pop(&mut self) -> u16 {
        self.stackpointer -= 1;
        self.stack[self.stackpointer as usize]
    }

    pub fn reset(&mut self) {
        self.programcounter = START_ADDRESS;
        self.memory = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.screen_changed = true;
        self.registers = [0; NUM_REGISTERS];
        self.indexregister = 0;
        self.stackpointer = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.load_characters();
        /*
        let result = self.display_sender.try_send(self.screen);
        match result {
            Err(_) => {}//log::error!("Channels is full, not sending screen"),
            _ => {}
        }*/
    }

    pub fn cpu_cycle(&mut self) {
        let opcode = self.get_opcode();
        self.execute_opcode(opcode);
    }

    fn execute_opcode(&mut self, op: u16) {
        // separate each digit 16bytes instruction into 4 parts of 4 bytes each
        let digit1 = (op & 0xF000) >> 12; // 0xF000 = 1111 0000 0000 0000, AND with 1 keeps the same, AND with 0 results always to 0
        let digit2 = (op & 0x0F00) >> 8; // 0x0F00 = 0000 1111 0000 0000
        let digit3 = (op & 0x00F0) >> 4; // 0x00F0 = 0000 0000 1111 0000
        let digit4 = op & 0x000F; // 0x000F = 0000 0000 0000 1111, no need for shifiting already on the far right

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
                self.screen_changed = true;
                /*let result = self.display_sender.try_send(self.screen);
                match result {
                    Err(_) => {}//log::error!("Channels is full, not sending screen"),
                    _ => {}
                }*/
            }
            (0, 0, 0xE, 0xE) => {
                self.programcounter = self.pop();
            }
            (1, a, b, c) => {
                //check to see if this actually works or not
                let addr = ((a << 8) | (b << 4) | c) as u16;
                self.programcounter = addr;
            }
            (2, a, b, c) => {
                let addr = ((a << 8) | (b << 4) | c) as u16;
                self.push(self.programcounter);

                self.programcounter = addr;
            }
            (3, x, k, k2) => {
                let vx = self.registers[x as usize];
                let kk = ((k << 4) | k2) as u8;
                if vx == kk {
                    self.programcounter += 2;
                }
            }
            (4, x, k, k2) => {
                let vx = self.registers[x as usize];
                let kk = ((k << 4) | k2) as u8;
                if vx != kk {
                    self.programcounter += 2;
                }
            }
            (5, x, y, 0) => {
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];
                if vx == vy {
                    self.programcounter += 2;
                }
            }
            (6, x, k, k2) => {
                let kk = (k << 4) | k2;
                self.registers[x as usize] = kk as u8;
            }
            (7, x, k, k2) => {
                let kk = (k << 4) | k2;
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(kk as u8);
            }
            (8, x, y, 0) => {
                self.registers[x as usize] = self.registers[y as usize];
            }
            // OR
            (8, x, y, 1) => {
                self.registers[x as usize] =
                    self.registers[x as usize] | self.registers[y as usize];
                self.registers[0xF] = 0;
            }
            // AND
            (8, x, y, 2) => {
                self.registers[x as usize] =
                    self.registers[x as usize] & self.registers[y as usize];
                self.registers[0xF] = 0;
            }
            // XOR
            (8, x, y, 3) => {
                self.registers[x as usize] =
                    self.registers[x as usize] ^ self.registers[y as usize];
                self.registers[0xF] = 0;
            }
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.registers[x].overflowing_add(self.registers[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.registers[x] = new_vx;
                self.registers[0xF] = new_vf;
            }
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.registers[x] = new_vx;
                self.registers[0xF] = new_vf;
            }
            (8, _, _, 6) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.variant == Variant::Legacy {
                    self.registers[x] = self.registers[y];
                }
                let lsb = self.registers[x] & 1;
                self.registers[x] >>= 1;
                self.registers[0xF] = lsb;
            }
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.registers[x] = new_vx;
                self.registers[0xF] = new_vf;
            }
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.variant == Variant::Legacy {
                    self.registers[x] = self.registers[y];
                }

                let msb = (self.registers[x] & 0b10000000) >> 7;
                self.registers[x] <<= 1;
                self.registers[0xF] = msb;
            }
            (9, x, y, 0) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.programcounter += 2;
                }
            }
            (0xA, _, _, _) => {
                let addr = op & 0b0000111111111111;
                self.indexregister = addr;
            }
            (0xB, _, _, _) => {
                match self.variant {
                    Variant::Legacy => {    
                        let addr = op & 0b0000111111111111;
                        self.programcounter = self.registers[0] as u16 + addr;
                    }
                    Variant::Modern => {
                        let addr = op & 0b0000111111111111;
                        let x = digit2 as usize;
                        self.programcounter = self.registers[x] as u16 + addr;
                    }
                };
            }
            (0xC, x, _, _) => {
                let kk = (op & 0b0000000011111111) as u8;
                let random_byte: u8 = rand::random();
                self.registers[x as usize] = random_byte & kk;
            }

            // draw call
            (0xD, x, y, n) => {
                let mut x_coord = self.registers[x as usize] as u16 % SCREEN_WIDTH as u16;
                let mut y_coord = self.registers[y as usize] as u16 % SCREEN_HEIGHT as u16;

                self.registers[0xF] = 0;

                let height = n as u16;
                let mut xoractived = false;

                for rownum in 0..height {
                    let sprite_byte = self.memory[(self.indexregister + rownum) as usize];
                    let y_coord_local = y_coord + rownum;
                    if y_coord_local >= SCREEN_HEIGHT as u16 {
                        break;
                    }
                    for colnum in 0..8 {
                        if (sprite_byte & (0b1000_0000 >> colnum)) == (0b1000_0000 >> colnum) {
                            //checks if byte at the colnum x position is 1
                            // bit is equal to one
                            // XOR when second bit is 1 it always flips the first bit
                            let x_coord_local = x_coord + colnum;
                            if x_coord_local >= SCREEN_WIDTH as u16 {
                                break;
                            }
                            let index =
                                (y_coord_local * SCREEN_WIDTH as u16 + x_coord_local) as usize; // calculate the index of 2d array in 1d

                            xoractived |= self.screen[index];

                            self.screen[index] ^= true;
                        }
                    }
                }
                self.registers[0xF] = if xoractived { 1 } else { 0 };
                self.screen_changed = true;

                /*let result = self.display_sender.try_send(self.screen);
                match result {
                    Err(_) => {}//log::error!("Channels is full, not sending screen"),
                    _ => {}
                }*/
                //log::info!("screen updated");
            }
            (0xE, x, 9, 0xE) => {
                let key = self.registers[x as usize] as usize;
                if self.keys[key] {
                    self.programcounter += 2;
                }
            }
            (0xE, x, 0xA, 1) => {
                let key = self.registers[x as usize] as usize;
                if !self.keys[key] {
                    self.programcounter += 2;
                }
            }
            (0xF, x, 0, 7) => {
                self.registers[x as usize] = self.delay_timer;
            }
            (0xF, x, 0, 0xA) => {
                let mut key_pressed = false;
                for i in 0..NUM_KEYS {
                    if self.keys[i] {
                        self.registers[x as usize] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }
                if !key_pressed {
                    log::info!("waiting for key press");
                    self.programcounter -= 2;
                    //repeat the instruction until a key is pressed
                }
            }
            (0xF, x, 1, 5) => {
                self.delay_timer = self.registers[x as usize];
            }
            (0xF, x, 1, 8) => {
                self.sound_timer = self.registers[x as usize];
            }
            (0xF, x, 1, 0xE) => {
                self.indexregister += self.registers[x as usize] as u16;
            }
            (0xF, x, 2, 9) => {
                self.indexregister =
                    FONTSET_START_ADDRESS as u16 + (self.registers[x as usize] as u16) * 5;
                // each sprite is 5 bytes long
            }
            (0xF, x, 3, 3) => {
                let vx = self.registers[x as usize];
                let hundreds = vx / 100; // this is will take on the whole number part of the result of the division
                let tens = (vx % 100) / 10;
                let ones = vx % 10;
                self.memory[self.indexregister as usize] = hundreds;
                self.memory[(self.indexregister + 1) as usize] = tens;
                self.memory[(self.indexregister + 2) as usize] = ones;
            }
            (0xF, x, 5, 5) => {
                for i in 0..=x {
                    self.memory[(self.indexregister + i) as usize] = self.registers[i as usize];
                }
                if self.variant == Variant::Legacy {
                    self.indexregister += x + 1;
                }
            }
            (0xF, x, 6, 5) => {
                for i in 0..=x {
                    self.registers[i as usize] = self.memory[(self.indexregister + i) as usize];
                }

                if self.variant == Variant::Legacy {
                    self.indexregister += x + 1;
                }
            }

            (_, _, _, _) => {
                log::error!("Unimplemented opcode: {:X}", op);
            } //unimplemented!("Unimplemented opcode: {:X}", op)},
        }
    }

    fn get_opcode(&mut self) -> u16 {
        let first_byte = self.memory[self.programcounter as usize] as u16;
        let second_byte = self.memory[self.programcounter as usize + 1] as u16;
        let opcode = first_byte << 8 | second_byte;
        // Shift the first byte by 8 bits and OR it with the second byte
        // OR with zeros on the right will make the second byte the last 8 bits unchanged
        // and the OR with the the same thing on both side will make the first byte the first 8 bits unchanged
        // combining two bytes into one 16 bit number
        self.programcounter += 2;
        opcode
    }

    pub fn timer_cycle(&mut self) {
        if self.delay_timer > 0 {
            // can not be equal to 0
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            // can not be equal to 0
            if self.sound_timer == 1 {
                log::warn!("BEEP BEEP BEEP");
                #[cfg(target_arch = "wasm32")]
                crate::audio::play();
            }
            self.sound_timer -= 1;
        }
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        self.keys[key] = pressed;
        if pressed {
            //log::info!("Key pressed: {}", key);
        }
    }
}
