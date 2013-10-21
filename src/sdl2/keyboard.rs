use scancode::ScanCode;
use keycode::KeyCode;
use std::num;

pub mod ffi {
    use std::libc::{uint32_t, int32_t, uint16_t};

    pub type SDL_Scancode = int32_t;
    pub type SDL_Keycode = int32_t;

    pub enum SDL_Keymod{
        KMOD_NONE = 0x0000,
        KMOD_LSHIFT = 0x0001,
        KMOD_RSHIFT = 0x0002,
        KMOD_LCTRL = 0x0040,
        KMOD_RCTRL = 0x0080,
        KMOD_LALT = 0x0100,
        KMOD_RALT = 0x0200,
        KMOD_LGUI = 0x0400,
        KMOD_RGUI = 0x0800,
        KMOD_NUM = 0x1000,
        KMOD_CAPS = 0x2000,
        KMOD_MODE = 0x4000,
        KMOD_RESERVED = 0x8000,
    }

    pub struct SDL_Keysym {
        scancode: SDL_Scancode,
        sym: SDL_Keycode,
        _mod: uint16_t,
        unused: uint32_t,
    }
}

pub enum Keymod {
    KMOD_None = ffi::KMOD_NONE as u16,
    KMOD_LShift = ffi::KMOD_LSHIFT as u16,
    KMOD_RShift = ffi::KMOD_RSHIFT as u16,
    KMOD_LCtrl = ffi::KMOD_LCTRL as u16,
    KMOD_RCtrl = ffi::KMOD_RCTRL as u16,
    KMOD_LAlt = ffi::KMOD_LALT as u16,
    KMOD_RAlt = ffi::KMOD_RALT as u16,
    KMOD_LGui = ffi::KMOD_LGUI as u16,
    KMOD_RGui = ffi::KMOD_RGUI as u16,
    KMOD_Num = ffi::KMOD_NUM as u16,
    KMOD_Caps = ffi::KMOD_CAPS as u16,
    KMOD_Mode = ffi::KMOD_MODE as u16,
    KMOD_Reserved = ffi::KMOD_RESERVED as u16,
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
