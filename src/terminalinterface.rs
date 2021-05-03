#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::collections::HashMap;
use crate::chip8gfx;
use crate::chip8kb;
use crate::sdlinterface;

use chip8kb::VIRTUAL_KEYS;
use chip8kb::DEFAULT_ASCII;

pub fn termgfxfact() -> Tgfx{
    unsafe {
        let win : *mut WINDOW = initscr();
        cbreak();
        noecho();
        nodelay(win, true);
        resize_term(32, 64);
        Tgfx::init(win)
    }
}

pub fn termkbfact(graphics: &Tgfx) -> Tkb{
    Tkb::init(graphics.win)
}

pub struct Tgfx {
    pub win:*mut WINDOW,
    pub screen: [[u8;64];32]
}

impl Tgfx {
    pub fn init(win: *mut WINDOW) -> Tgfx {
        unsafe {
            return Tgfx{win: win, screen: [[0;64];32]};
        }
    }

    pub fn update(&self) {
        for y in 0..self.screen.len() {
            for x in 0..self.screen[y].len() {
                //TODO: Replace this with something more efficient.
                //and less shitty.
                unsafe {mvwaddch(self.win, y as i32, x as i32, if self.screen[y][x] == 0 {' ' as u32} else {('0' as u32)|0x00010000});}
            }
        }
        unsafe {
            refresh();
        };
    }

    fn dbg_print_screen(&self){
        for y in 0..self.screen.len() {
            for x in 0..self.screen[y].len() {
                eprint!("{}", if self.screen[y][x] > 0 {"█"} else {"."});
            }
            eprintln!();
        }
        eprintln!();
    }
}

impl chip8gfx::Interface for Tgfx {
    //return 01 if any set pixels are changed to unset, and 00 otherwise
    fn draw_sprite(&mut self, x:u8, y:u8, sprite:&[u8]) -> u8 {
        let mut set = 0x00;
        for i in 0..sprite.len(){
            if y as usize + i >= 32 {break;}
            for pos in 0..8{
                if x as usize + pos >= 64 {break;}
                let bit = (sprite[i] >> (7-pos)) & 0x01;
                if bit == 0x01 && self.screen[y as usize + i][x as usize + pos] > 0{
                    set = 0x01;
                }
                self.screen[y as usize + i][x as usize + pos] = bit^self.screen[y as usize + i][x as usize + pos];
            }
        }
        //self.dbg_print_screen();
        return set;
    }

    fn clear_screen(&mut self) {
        for l in self.screen.iter_mut(){
            for i in 0..l.len(){
                l[i] = 0;
            }
        }
    }

}

pub struct Tkb {
    pub fwdmap: HashMap<::std::os::raw::c_int, u8>,
    pub backmap: HashMap<u8, Vec<::std::os::raw::c_int>>,
    pub win: *mut WINDOW
}

impl chip8kb::Interface for Tkb {
    fn update(&self) -> u16 {
        let mut setkeys:u16 = 0;
        let mut keypress = 0;
        while keypress != -1 {
            unsafe {
                keypress = wgetch(self.win);
                match self.fwdmap.get(&keypress) {
                    None => (),
                    Some(keyID) => {
                        setkeys |= 0x1u16 << keyID;
                    }
                }
            }
        }
        return setkeys;
    }
}

/*
Keyboard layout:
|1|2|3|C|
|4|5|6|D|
|7|8|9|E|
|A|0|B|F|

*/

impl Tkb {
    pub fn init(win: *mut WINDOW) -> Tkb {
        let backmap: HashMap<u8, Vec<::std::os::raw::c_int>> = 
        VIRTUAL_KEYS.iter().cloned()
            .zip(DEFAULT_ASCII.iter()
            .map(|&e| vec![e as ::std::os::raw::c_int]))
            .collect();
        let fwdmap: HashMap<::std::os::raw::c_int, u8> = 
        DEFAULT_ASCII.iter()
            .map(|&e| e as ::std::os::raw::c_int)
            .zip(VIRTUAL_KEYS.iter().copied())
            .collect();
        let keyboard = Tkb{
            fwdmap: fwdmap,
            backmap: backmap,
            win: win
        };
        return keyboard;
    }

    pub fn set_key_assoc(&mut self, real_key: char, virtual_key: u8){
        if virtual_key > 0xF {
            return
        }
        match self.fwdmap.get(&(real_key as ::std::os::raw::c_int)) {
            Some(vkey) => {   
                match self.backmap.get_mut(&vkey) {
                    None => panic!("Desynced keymaps!"), //This should never happen
                    Some(keylist) => {
                        keylist.retain(|&x| x != real_key as ::std::os::raw::c_int);
                    }
                }
            },
            None => ()}
        self.fwdmap.insert(real_key as ::std::os::raw::c_int, virtual_key);
        match self.backmap.get_mut(&virtual_key) {
            Some(keylist) => keylist.push(real_key as ::std::os::raw::c_int),
            None => ()
        }
    }
}

