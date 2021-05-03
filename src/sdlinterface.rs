#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::terminalinterface;
use crate::chip8gfx;
use crate::chip8kb;

pub struct SDLgfx {

}

pub struct SDLkb {

}

impl chip8gfx::Interface for SDLgfx {
    fn draw_sprite(&mut self, x:u8, y:u8, sprite:&[u8]) -> u8{
        return 0x00
    }

    fn clear_screen(&mut self){
        
    }


}