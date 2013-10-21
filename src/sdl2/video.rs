use error::*;
use std::ptr;

mod error;

pub mod ffi {
    use std::libc::{c_int, uint32_t, c_char};
    pub struct SDL_Window;

    pub enum SDL_WindowFlags {
        SDL_WINDOW_FULLSCREEN = 0x00000001,
        SDL_WINDOW_OPENGL = 0x00000002,
        SDL_WINDOW_SHOWN = 0x00000004,
        SDL_WINDOW_HIDDEN = 0x00000008,
        SDL_WINDOW_BORDERLESS = 0x00000010,
        SDL_WINDOW_RESIZABLE = 0x00000020,
        SDL_WINDOW_MINIMIZED = 0x00000040,
        SDL_WINDOW_MAXIMIZED = 0x00000080,
        SDL_WINDOW_INPUT_GRABBED = 0x00000100,
        SDL_WINDOW_INPUT_FOCUS = 0x00000200,
        SDL_WINDOW_MOUSE_FOCUS = 0x00000400,
        SDL_WINDOW_FULLSCREEN_DESKTOP = 0x00001001,
        SDL_WINDOW_FOREIGN = 0x00000800
    }

    externfn!(fn SDL_CreateWindow(title: *c_char, 
                                  x: c_int, y: c_int, 
                                  w: c_int, h: c_int, 
                                  flags: uint32_t) -> *SDL_Window)

    externfn!(fn SDL_DestroyWindow(window: *SDL_Window))
    externfn!(fn SDL_GetWindowFromID(id: uint32_t) -> *SDL_Window)
}

pub enum WindowFlags {
    FullScreen = ffi::SDL_WINDOW_FULLSCREEN as u32,
    OpenGL = ffi::SDL_WINDOW_OPENGL as u32,
    Shown = ffi::SDL_WINDOW_SHOWN as u32,
    Hidden = ffi::SDL_WINDOW_HIDDEN as u32,
    Borderless = ffi::SDL_WINDOW_BORDERLESS as u32,
    Resizable = ffi::SDL_WINDOW_RESIZABLE as u32,
    Minimized = ffi::SDL_WINDOW_MINIMIZED as u32,
    Maximized = ffi::SDL_WINDOW_MAXIMIZED as u32,
    InputGrabbed = ffi::SDL_WINDOW_INPUT_GRABBED as u32,
    InputFocus = ffi::SDL_WINDOW_INPUT_FOCUS as u32,
    MouseFocus = ffi::SDL_WINDOW_MOUSE_FOCUS as u32,
    FullScreenDesktop = ffi::SDL_WINDOW_FULLSCREEN_DESKTOP as u32,
    Foreign = ffi::SDL_WINDOW_FOREIGN as u32,
}

pub struct Window {
    raw: *ffi::SDL_Window,
    owned: bool,
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::SDL_DestroyWindow(self.raw);
            }
        }
    }
}

impl Window {
    pub fn new(title: &str, 
               x: i32, y: i32,
               w: i32, h: i32,
               flags: &[WindowFlags]) -> Result<~Window, ~str> {
        let f = do flags.iter().fold(0) |all, it| {
            all | *it as u32
        };

        unsafe {
            let raw = do title.with_c_str |buf| {
                ffi::SDL_CreateWindow(buf, x, y, w, h, f)
            };

            if ptr::is_null(raw) {
                Err(get_error())
            } else {
                Ok(~Window{ raw: raw, owned: true })
            }
        }
    }

    pub fn get_from_id(id: u32) -> Result<~Window, ~str> {
        unsafe {
            let raw = ffi::SDL_GetWindowFromID(id);

            if ptr::is_null(raw) {
                Err(get_error())
            } else {
                Ok(~Window{raw: raw, owned: false})
            }
        }
    }
}

