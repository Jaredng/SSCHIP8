extern crate keyboard_query;

use std::collections::HashMap;
use crate::chip8kb::{self, VIRTUAL_KEYS, DEFAULT_WIN_KEYCODES};

use keyboard_query::{DeviceQuery, DeviceState};

pub struct minimalkb {
    device_state: DeviceState,
    keymap: HashMap<u16, u8>, // Map from phys keys to virtual keys
}

impl minimalkb {
    pub fn init() -> minimalkb {
        minimalkb {
            device_state : DeviceState::new(),
            keymap : minimalkb::make_keymap(&VIRTUAL_KEYS,&DEFAULT_WIN_KEYCODES)
        }
    }

    fn make_keymap(virt_keys:&[u8;16], phys_keys:&[u16;16]) -> HashMap<u16,u8> {
        phys_keys.iter().cloned().zip(virt_keys.iter().cloned()).collect()
    }

}

impl chip8kb::Interface for minimalkb {
    fn update(&self) -> u16 {
        let keys = self.device_state.get_keys();
        let mut pressed:u16 = 0;
        for key in keys.iter() {
            match self.keymap.get(key)  {
                None => (),
                Some (keyID) => pressed |= 0x1u16 << keyID
            }
        }
        return pressed
    }
}