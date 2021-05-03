//Snake case really doesn't make sense for how I'm naming processor functions.
#![allow(non_snake_case)]
#![feature(asm)]

use std::process::exit;
use std::env;
use std::time::{Instant};
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use crate::chip8kb::Interface;

mod chip8gfx;
mod chip8kb;
mod terminalinterface;
mod sdlinterface;
mod minimalkbinterface;

extern crate rand;

const CLOCK_DEFAULT: u64 = 500;

macro_rules! nibs_to_addr {
    ($hi:expr, $mid:expr, $lo:expr) => {
        (($hi as u16) << 8) | (($mid as u16) << 4) | ($lo as u16);
    };
}

macro_rules! nibs_to_byte {
    ($hi:expr, $lo:expr) => {
        (($hi << 4) | $lo)
    };
}

pub struct CHIP8cpu{
    // index register
    pub i: u16,
    // program counter
    pub pc: u16,
    // memory
    pub memory: [u8; 4096],
    // registers
    pub v: [u8; 16],
    // stack
    pub stack: [u16; 255],
    // stack pointer
    pub sp: u8,
    // delay timer
    pub dt: u64,
    // sound timer
    pub st: u64,
    //clock speed
    pub clock: u64,
    // current instruction in nibs
    pub ins: [u8; 4],
    //CPU Mode --
    //0x00: chip-8 spec
    //0x01: alt shl/shr commands
    pub mode: u8
    //Currently pressed keys
}

impl CHIP8cpu {
    fn cycle(&mut self, gfx: &mut dyn chip8gfx::Interface, kb: u16) -> bool {
        self.ticktimers();
        //TODO: Implement timer decrements
        self.ins[0] = (self.memory[self.pc as usize] >> 4) & 0xF;
        self.ins[1] = (self.memory[self.pc as usize]) & 0xF;
        self.pc += 1;
        self.ins[2] = (self.memory[self.pc as usize] >> 4) & 0xF;
        self.ins[3] = (self.memory[self.pc as usize]) & 0xF;
        self.pc += 1;

        match self.ins[0] {
            0x0 => match self.ins[1] {
                0x0 => match self.ins[3] {
                    0x0 => self.i00E0(gfx),
                    0xE => self.i00EE(),
                    _ => self.instr_panic()
                }
                _ => self.i0NNN()
            }
            0x1 => self.i1NNN(),
            0x2 => self.i2NNN(),
            0x3 => self.i3XNN(),
            0x4 => self.i4XNN(),
            0x5 => self.i5XY0(),
            0x6 => self.i6XNN(),
            0x7 => self.i7XNN(),
            0x8 => match self.ins[3] {
                0x0 => self.i8XY0(),
                0x1 => self.i8XY1(),
                0x2 => self.i8XY2(),
                0x3 => self.i8XY3(),
                0x4 => self.i8XY4(),
                0x5 => self.i8XY5(),
                0x6 => match self.mode {
                    0x00 => self.i8XY6(),
                    0x01 => self.i8XY6_alt(),
                    _ => panic!("Unsupported mode!")
                }
                0x7 => self.i8XY7(),
                0xE => match self.mode {
                    0x00 => self.i8XYE(),
                    0x01 => self.i8XYE_alt(),
                    _ => panic!("Unsupported mode!")
                } 
                _ => self.instr_panic()
            }
            0x9 => self.i9XY0(),
            0xA => self.iANNN(),
            0xB => self.iBNNN(),
            0xC => self.iCXNN(),
            0xD => self.iDXYN(gfx),
            0xE => match self.ins[3] {
                0xE => self.iEX9E(kb),
                0x1 => self.iEXA1(kb),
                _ => self.instr_panic()
            }
            0xF => match self.ins[2] {
                0x0 => match self.ins[3]{
                    0x7 => self.iFX07(),
                    0xA => self.iFX0A(kb),
                    _ => self.instr_panic()
                }
                0x1 => match self.ins[3]{
                    0x5 => self.iFX15(),
                    0x8 => self.iFX18(),
                    0xE => self.iFX1E(),
                    _ => self.instr_panic()
                }
                0x2 => self.iFX29(),
                0x3 => self.iFX33(),
                0x5 => self.iFX55(),
                0x6 => self.iFX65(),
                _ => self.instr_panic()
            }
            _ => self.instr_panic()
        }

        return true;
    }

    fn instr_panic(&mut self){
        panic!("Invalid instruction: {:X?} at address {:X?}", self.ins, self.pc)
    }

    fn ticktimers(&mut self){
        if self.st > 0 { self.st -= 1;}
        if self.dt > 0 { self.dt -= 1;}
    }

    //Clear screen
    fn i00E0(&mut self, gfx: &mut dyn chip8gfx::Interface){
        gfx.clear_screen();
    }

    //Return from subroutine
    fn i00EE(&mut self){
        self.sp -= 1;
        if self.sp == 255 {
            panic!("Stack Underflow with instr {:X?} at address {:X?}", self.ins, self.pc);
        }
        self.pc = self.stack[self.sp as usize];
    }
    
    //Execute machine language subroutine at 0xNNN
    fn i0NNN(&mut self){
        self.i2NNN();
    }

    //Jump to 0xNNN
    fn i1NNN(&mut self){
        self.pc = nibs_to_addr!(self.ins[1], self.ins[2], self.ins[3]);
    }

    //Execute subroutine at 0xNNN
    fn i2NNN(&mut self){
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        if self.sp == 0 {
            panic!("Stack Overflow with instr {:X?} at address {:X?}", self.ins, self.pc);
        }
        self.pc = nibs_to_addr!(self.ins[1], self.ins[2], self.ins[3]);
    }

    //Skip the following instruction if the value of register VX equals NN
    fn i3XNN(&mut self){
        if self.v[self.ins[1] as usize] == nibs_to_byte!(self.ins[2],self.ins[3]) {
            self.pc += 2;
        }
    }

    //Skip the following instruction if the value of register VX is not equal to NN
    fn i4XNN(&mut self){
        if self.v[self.ins[1] as usize] != nibs_to_byte!(self.ins[2],self.ins[3]) {
            self.pc += 2;
        }
    }

    //Skip the following instruction if the value of register VX is equal to the value of register VY
    fn i5XY0(&mut self){
        if self.v[self.ins[1] as usize] == self.v[self.ins[2] as usize] {
            self.pc += 2;
        }
    }

    //Store number NN in register VX
    fn i6XNN(&mut self){
        self.v[self.ins[1] as usize] = nibs_to_byte!(self.ins[2],self.ins[3]);
    }

    //Add the value NN to register VX
    fn i7XNN(&mut self){
        self.v[self.ins[1] as usize] += nibs_to_byte!(self.ins[2],self.ins[3]);
    }

    //Store the value of register VY in register VX
    fn i8XY0(&mut self){
        self.v[self.ins[1] as usize] = self.v[self.ins[2] as usize];
    }

    //Set VX to VX OR VY
    fn i8XY1(&mut self){
        self.v[self.ins[1] as usize] = self.v[self.ins[1] as usize] | self.v[self.ins[2] as usize];
    }

    //Set VX to VX AND VY
    fn i8XY2(&mut self){
        self.v[self.ins[1] as usize] = self.v[self.ins[1] as usize] & self.v[self.ins[2] as usize];
    }

    //Set VX to VX XOR VY
    fn i8XY3(&mut self){
        self.v[self.ins[1] as usize] = self.v[self.ins[1] as usize] ^ self.v[self.ins[2] as usize];
    }

    // Add the value of register VY to register VX
    // Set VF to 01 if a carry occurs
    // Set VF to 00 if a carry does not occur
    fn i8XY4(&mut self){
        let sum:u16 = self.v[self.ins[1] as usize] as u16 + self.v[self.ins[2] as usize] as u16;
        if sum > u8::max_value() as u16{
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
        self.v[self.ins[1] as usize] = sum as u8;
    }

    // Subtract the value of register VY from register VX
    // Set VF to 00 if a borrow occurs
    // Set VF to 01 if a borrow does not occur
    fn i8XY5(&mut self){
        let diff:i16 = self.v[self.ins[1] as usize] as i16 - self.v[self.ins[2] as usize] as i16;
        if diff < 0 {
            self.v[0xF] = 0;
        } else {
            self.v[0xF] = 1;
        }
        self.v[self.ins[1] as usize] = self.v[self.ins[1] as usize] - self.v[self.ins[2] as usize];
    }

    // Store the value of register VY shifted right one bit in register VX¹
    // Set register VF to the least significant bit prior to the shift
    // VY is unchanged
    fn i8XY6(&mut self){
        self.v[0xF] = 0x01 & self.v[self.ins[2] as usize];
        self.v[self.ins[1] as usize] = self.v[self.ins[2] as usize] >> 1;
    }

    // Store the value of register VX shifted right one bit in register VX¹
    // Set register VF to the least significant bit prior to the shift
    fn i8XY6_alt(&mut self){
        self.v[0xF] = 0x01 & self.v[self.ins[1] as usize];
        self.v[self.ins[1] as usize] = self.v[self.ins[1] as usize] >> 1;
    }

    // Set register VX to the value of VY minus VX
    // Set VF to 00 if a borrow occurs
    // Set VF to 01 if a borrow does not occur
    fn i8XY7(&mut self){
        let diff:i16 = self.v[self.ins[2] as usize] as i16 - self.v[self.ins[1] as usize] as i16;
        if diff < 0 {
            self.v[0xF] = 0;
        } else {
            self.v[0xF] = 1;
        }
        self.v[self.ins[1] as usize] = self.v[self.ins[2] as usize] - self.v[self.ins[1] as usize];
    }

    // Store the value of register VY shifted left one bit in register VX¹
    // Set register VF to the most significant bit prior to the shift
    // VY is unchanged
    fn i8XYE(&mut self){
        self.v[0xF] = (0x80 & self.v[self.ins[2] as usize]) >> 7;
        self.v[self.ins[1] as usize] = self.v[self.ins[2] as usize] << 1;
    }

    // Store the value of register VX shifted left one bit in register VX¹
    // Set register VF to the most significant bit prior to the shift
    fn i8XYE_alt(&mut self){
        self.v[0xF] = (0x80 & self.v[self.ins[1] as usize]) >> 7;
        self.v[self.ins[1] as usize] = self.v[self.ins[1] as usize] << 1;
    }

    //Skip the following instruction if the value of register VX is not equal to the value of register VY
    fn i9XY0(&mut self){
        if self.v[self.ins[1] as usize] != self.v[self.ins[2] as usize] {
            self.pc += 2;
        }
    }

    //Store memory address NNN in register I
    fn iANNN(&mut self){
        self.i = nibs_to_addr!(self.ins[1],self.ins[2],self.ins[3])
    }

    //Jump to address NNN + V0
    fn iBNNN(&mut self){
        self.pc = nibs_to_addr!(self.ins[1], self.ins[2], self.ins[3]) + self.v[0x0] as u16;
    }

    //Set VX to a random number with a mask of NN
    fn iCXNN(&mut self){
        self.v[self.ins[1] as usize] = rand::random::<u8>() & nibs_to_byte!(self.ins[2],self.ins[3]);
    }

    // Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
    // Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
    fn iDXYN(&mut self, gfx: &mut dyn chip8gfx::Interface){
        self.v[0xF] = gfx.draw_sprite(self.v[self.ins[1] as usize],self.v[self.ins[2] as usize], 
                        &self.memory[self.i as usize .. self.i as usize + self.ins[3] as usize]);
    }

    //Skip the following instruction if the key corresponding to the 
    //hex value currently stored in register VX is pressed
    fn iEX9E(&mut self, kb: u16){
        if (kb >> (self.v[self.ins[1] as usize]) & 0x1) == 1 {
            self.pc += 2;
        }
    }

    //Skip the following instruction if the key corresponding to the 
    //hex value currently stored in register VX is not pressed
    fn iEXA1(&mut self, kb: u16){
        if (kb >> (self.v[self.ins[1] as usize]) & 0x1) != 1 {
            self.pc += 2;
        }
    }

    // Store the current value of the delay timer in register VX
    fn iFX07(&mut self){
        self.v[self.ins[1] as usize] = ((self.dt * 60)/self.clock) as u8;
    }

    // Wait for a keypress and store the result in register VX
    fn iFX0A(&mut self, kb: u16){
        if kb > 0 { // if keypress, store in VX
            self.v[self.ins[1] as usize] = Self::countTrailingZeros(kb as u32) as u8;
        } else { // otherwise decrement by 2
            self.pc -= 2;
        }

    }

    //Set the delay timer to the value of register VX
    fn iFX15(&mut self){
        //multiply by clock to help correct for clock speed
        self.dt = (self.v[self.ins[1] as usize] as u64 * self.clock) / 60;
    }

    //Set the sound timer to the value of register VX
    fn iFX18(&mut self){
        //multiply by 60 to help correct for clock speed
        self.st = (self.v[self.ins[1] as usize] as u64 * self.clock) / 60;
    }

    //Add the value stored in register VX to register I
    fn iFX1E(&mut self){
        self.i += self.v[self.ins[1] as usize] as u16;
    }

    //Set I to the memory address of the sprite data corresponding to 
    //the hexadecimal digit stored in register VX
    //TODO: Add these hex digits on init
    fn iFX29(&mut self){
        self.i = self.v[self.ins[1] as usize] as u16 * 5;
    }

    //Store the binary-coded decimal equivalent of the 
    //value stored in register VX at addresses I, I + 1, and I + 2
    fn iFX33(&mut self){
        let val = self.v[self.ins[1] as usize];
        self.memory[self.i as usize] = val / 100;
        self.memory[self.i as usize + 1] = (val / 10) % 10;
        self.memory[self.i as usize + 2] = val % 10;
    }

    // Store the values of registers V0 to VX inclusive in memory starting at address I
    // I is set to I + X + 1 after operation
    fn iFX55(&mut self){
        for j in 0..(self.ins[1] as usize + 1)  {
            self.memory[self.i as usize] = self.v[j];
            self.i += 1;
        }
    }

    // Fill registers V0 to VX inclusive with the values stored in memory starting at address I
    // I is set to I + X + 1 after operation²
    fn iFX65(&mut self){
        for j in 0..(self.ins[1] as usize + 1)  {
            self.v[j] = self.memory[self.i as usize];
            self.i += 1;
        }
    }

    //maximum efficiency.
    fn countTrailingZeros(value: u32) -> u32{
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            unsafe {
                asm!(
                    "rbit {0}, {0}",
                    "clz  {0}, {0}",
                    inout(reg) value,
                );
            }
            return value;
        }
        #[cfg(any(target_arch = "x86_64"))]
        {
            unsafe {
                asm!(
                    "bsf {0}, {0}",
                    inout(reg) value,
                );
            }
            return value;
        }
        #[cfg(all(not(target_arch = "aarch64"), not(target_arch="arm"), not(target_arch = "x86_64")))]
        {
            if value == 0 {return 32};
            const DE_BRUIJN_SEQUENCE:[u32;32] = 
            [
                0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8, 
                31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9
            ];
            return DE_BRUIJN_SEQUENCE[(((value & (0 - value)) * 0x077CB531u32) >> 27) as usize];
        }
    }
    
}

fn main() {
    // SSCHIP-8 [romfile]
    let args: Vec<String> = env::args().collect();
    let clockspeed = CLOCK_DEFAULT;
    // load romfile & validate (check size & fmt, also catch filesystem errors etc.)

    // do general CPU init
    //TODO: Initialize hex sprites
    if args.len() < 2 { 
        println!("Missing rom file!");
        exit(6);
    }

    let commands = &args[1..args.len() - 1];

    //SHL/SHR use VY by default (VX = VY >> 1). Altmode ignores VY (VX = VX >> 1).
    let mut mode = 0x00;

    if commands.len() > 0 {
        if commands[0] == "altmode" {
            mode = 0x01;
        }

    }

    if commands.len() > 1 {
        if commands[1] == "sdl" {

        }
    }

    let filname = &args[args.len() - 1];

    match fs::metadata(filname) {
        Err(why) => panic!("coulnd't read file metadata: {}", why),
        Ok(metadata) => if metadata.len() > 3584 || metadata.len() < 1 {
            println!("File size issue!");
            exit(6);
        }
    };

    let romfil =  match File::open(filname) {
        Err(why) => panic!("couldn't open {}: {}", &args[1], why),
        Ok(file) => file,
    };

    //Make init structure branch when time for SDL implementation
    let mut graphics = terminalinterface::termgfxfact();
    let keyboard = minimalkbinterface::minimalkb::init();

    let mut cpu = CHIP8cpu {
        i: 0x0,
        pc: 0x200,
        memory: [0x0; 4096],
        v: [0x0; 16],
        stack: [0x0; 255],
        sp: 0,
        dt: 0,
        st: 0,
        clock: clockspeed,
        ins: [0x0; 4],
        mode: mode
    };

    // init default sprites
    for i in 0..SPRITES.len() {
        cpu.memory[i] = SPRITES[i];
    }

    // initialize rom from memory address $200
    let mut adr = 0x200;
    for b in romfil.bytes(){
        if b.is_err(){
            panic!("File read error at {}", b.unwrap_err())
        }
        cpu.memory[adr] = b.unwrap();
        adr += 1;
    }

    // main program loop
    let mut keys;
    let mut cycles = 0;
    let mut spares = 0;
    let mut prev = Instant::now();
    let ticksize = 1000000 / clockspeed; //microseconds per clock
    loop {
        let cur = Instant::now();
        let dur = cur.saturating_duration_since(prev).as_micros() as u64;
        if dur + spares > ticksize {
            cycles = (dur + spares) / ticksize;
            spares = (dur + spares) % ticksize;
            prev = cur;
        }
        //TODO -- Video & Audio updates
        for _ in 0..cycles{
            keys = keyboard.update();
            cpu.cycle(&mut graphics, keys); 
            graphics.update();
        }
        cycles = 0;
    }

}

const SPRITES:[u8;80] = [
    //0
    0b11110000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11110000,
    //1
    0b00100000,
    0b01100000,
    0b00100000,
    0b00100000,
    0b01110000,
    //2
    0b11110000,
    0b00010000,
    0b11110000,
    0b10000000,
    0b11110000,
    //3
    0b11110000,
    0b00010000,
    0b11110000,
    0b00010000,
    0b11110000,
    //4
    0b10010000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b00010000,
    //5
    0b11110000,
    0b10000000,
    0b11110000,
    0b00010000,
    0b11110000,
    //6
    0b11110000,
    0b10000000,
    0b11110000,
    0b10010000,
    0b11110000,
    //7
    0b11110000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b01000000,
    //8
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b11110000,
    //9
    0b11110000,
    0b10010000,
    0b11110000,
    0b00010000,
    0b11110000,
    //A
    0b11110000,
    0b10010000,
    0b11110000,
    0b10010000,
    0b10010000,
    //B
    0b11100000,
    0b10010000,
    0b11100000,
    0b10010000,
    0b11100000,
    //C
    0b11110000,
    0b10000000,
    0b10000000,
    0b10000000,
    0b11110000,
    //D
    0b11100000,
    0b10010000,
    0b10010000,
    0b10010000,
    0b11100000,
    //E
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b11110000,
    //F
    0b11110000,
    0b10000000,
    0b11110000,
    0b10000000,
    0b10000000
];

//Rewrap function for Wrapping<T> if I decide to go with that. 
//Probably just gonna use the nightly compiler tho.
// fn rewrap<T,U>(Wrapping(t) : Wrapping<T>) -> Wrapping<U> where T: TryInto<U> {
//     Wrapping(t.try_into().ok().unwrap())
// }