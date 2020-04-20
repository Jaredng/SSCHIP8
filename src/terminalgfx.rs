#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::chip8gfx;

pub struct Tgfx {

}

impl Tgfx {
    pub fn init() -> Box<Tgfx> {
        return Box::new(Tgfx{});
    }
}

impl chip8gfx::Interface for Tgfx {
    fn draw_sprite(&self, x:u8, y:u8, sprite:&[u8]) -> u8 {
        return 0x00
    }

    fn clear_screen(&self) {

    }
}