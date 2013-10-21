use scancode::ScanCode;
use keycode::KeyCode;
use std::num;

pub mod ffi {
    use std::libc::{uint32_t, int32_t, uint16_t};

    pub type SDL_Scancode = int32_t;
    pub type SDL_Keycode = int32_t;

    pub struct SDL_Keysym {
        scancode: SDL_Scancode,
        sym: SDL_Keycode,
        _mod: uint16_t,
        unused: uint32_t,
    }
}

pub enum Keymod {
    KMOD_None     = 0x0000,
    KMOD_LShift   = 0x0001,
    KMOD_RShift   = 0x0002,
    KMOD_LCtrl    = 0x0040,
    KMOD_RCtrl    = 0x0080,
    KMOD_LAlt     = 0x0100,
    KMOD_RAlt     = 0x0200,
    KMOD_LGui     = 0x0400,
    KMOD_RGui     = 0x0800,
    KMOD_Num      = 0x1000,
    KMOD_Caps     = 0x2000,
    KMOD_Mode     = 0x4000,
    KMOD_Reserved = 0x8000,
}

impl Keymod {
    pub fn wrap(key_mod: u16) -> ~[Keymod] {
        let mut mods: ~[Keymod] = ~[];
        let all_mods = [
            KMOD_None,
            KMOD_LShift,
            KMOD_RShift,
            KMOD_LCtrl,
            KMOD_RCtrl,
            KMOD_LAlt,
            KMOD_RAlt,
            KMOD_LGui,
            KMOD_RGui,
            KMOD_Num,
            KMOD_Caps,
            KMOD_Mode,
            KMOD_Reserved,
            ];
        for m in all_mods.iter() {
            if key_mod & *m as u16 != 0 {
                mods.push(*m);
            }
        }

        mods
    }
}


pub struct Keysym {
    scancode: ScanCode,
    sym: KeyCode,
    _mod: ~[Keymod],    
}

impl Keysym {
    pub fn wrap(sym: &ffi::SDL_Keysym) -> ~Keysym {
        ~Keysym{ scancode: num::from_i32(sym.scancode).unwrap(), 
            sym: num::from_i32(sym.sym).unwrap(),
            _mod: Keymod::wrap(sym._mod) }

    }
}
