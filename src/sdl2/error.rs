use std::str;

pub mod ffi {
    use std::libc::c_char;

    extern {
        pub fn SDL_ClearError();
        pub fn SDL_GetError() -> *c_char;
        pub fn SDL_SetError(err: *c_char);
    }
}

pub fn clear_error() {
    unsafe {
        ffi::SDL_ClearError();
    }
}

pub fn get_error() -> ~str {
    unsafe {
        let cstr = ffi::SDL_GetError();
        str::raw::from_c_str(cstr)
    }
}

pub fn set_error(err: &str) {
    do err.with_c_str |buf| {
        unsafe { ffi::SDL_SetError(buf); }
    }
}

