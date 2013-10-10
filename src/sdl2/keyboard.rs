mod scancode;
mod keycode;

pub mod ffi {
    use std::libc::{uint32_t, int32_t, uint16_t};

    pub type SDL_Scancode = int32_t;
    pub type SDL_Keycode = int32_t;

    pub struct SDL_Keysym {
        scancode: SDL_Scancode,
        sym: SDL_Keycode,
        _mod: uint16_t,
        unised: uint32_t,
    }

