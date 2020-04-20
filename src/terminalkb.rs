#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::chip8kb;

pub struct Tkb {

}

impl chip8kb::Interface for Tkb {
        //Return true if given key is down when function called. False otherwise.
        fn check_pressed(&self, key: u8) -> bool{
            return false;
        }

        //Wait for the next keypress, then return its key ID
        fn get_keypress(&self) -> u8{
            return 0x00;
        }
}

impl Tkb {
    pub fn init() -> Box<Tkb> {
        return Box::new(Tkb{});
    }
}