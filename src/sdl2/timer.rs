pub mod ffi {
    use std::libc::uint32_t;

    extern {
        pub fn SDL_Delay(ms: uint32_t);
    }
}

pub fn delay(ms: u32) {
    unsafe { ffi::SDL_Delay(ms); }
}
