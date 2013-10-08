use std::str;
mod ffi;

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

