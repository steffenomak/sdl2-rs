use error::*;
use std::ptr;

pub mod ffi {
    use std::libc::{c_int, uint32_t, c_char};
    pub struct SDL_Window;

    externfn!(fn SDL_CreateWindow(title: *c_char, 
                                  x: c_int, y: c_int, 
                                  w: c_int, h: c_int, 
                                  flags: uint32_t) -> *SDL_Window)

    externfn!(fn SDL_DestroyWindow(window: *SDL_Window))
    externfn!(fn SDL_GetWindowFromID(id: uint32_t) -> *SDL_Window)
    externfn!(fn SDL_GL_GetSwapInterval() -> c_int)
}

pub enum WindowFlags {
    FullScreen          = 0x00000001,
    OpenGL              = 0x00000002,
    Shown               = 0x00000004,
    Hidden              = 0x00000008,
    Borderless          = 0x00000010,
    Resizable           = 0x00000020,
    Minimized           = 0x00000040,
    Maximized           = 0x00000080,
    InputGrabbed        = 0x00000100,
    InputFocus          = 0x00000200,
    MouseFocus          = 0x00000400,
    FullScreenDesktop   = 0x00001001,
    Foreign             = 0x00000800,
}

#[deriving(FromPrimitive)]
pub enum WindowEventID {
    NoneEvent        = 0_u8,
    ShownEvent       = 1_u8,
    HiddenEvent      = 2_u8,
    ExposedEvent     = 3_u8,
    MovedEvent       = 4_u8,
    ResizedEvent     = 5_u8,
    SizeChangedEvent = 6_u8,
    MinimizedEvent   = 7_u8,
    MaximizedEvent   = 8_u8,
    RestoredEvent    = 9_u8,
    EnterEvent       = 10_u8,
    LeaveEvent       = 11_u8,
    FocusGainedEvent = 12_u8,
    FocusLostEvent   = 13_u8,
    CloseEvent       = 14_u8,
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

    pub fn get_swap_interval() -> Result<bool, ~str> {
        let i = unsafe {
            ffi::SDL_GL_GetSwapInterval()
        };

        match i {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(get_error()),
        }
    }
}

